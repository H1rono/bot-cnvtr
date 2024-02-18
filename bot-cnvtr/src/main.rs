use std::{net::SocketAddr, sync::Arc};

use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

use bot::BotImpl;
use repository::RepositoryImpl;
use router::make_router;
use traq_client::ClientImpl;
use wh_handler::WebhookHandlerImpl;

use bot_cnvtr::config::ConfigComposite;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or("info".into()))
        .init();
    let ConfigComposite {
        bot_config,
        router_config,
        client_config,
        repo_config,
    } = ConfigComposite::from_env()
        .map_err(anyhow::Error::from)
        .or_else(|_| -> anyhow::Result<ConfigComposite> {
            dotenvy::from_filename_override(".env")?;
            Ok(ConfigComposite::from_env()?)
        })?;

    let client = ClientImpl::new(&client_config.bot_access_token);
    let repo = RepositoryImpl::connect(&repo_config.database_url()).await?;
    repo.migrate().await?;
    let (tx, rx) = cron::channel(100);
    let infra = bot_cnvtr::infra::InfraImpl::new_wrapped(repo, client, tx);
    let infra = Arc::new(infra);

    // run notifier in background
    let handle = {
        let infra = infra.clone();
        tokio::task::spawn(async move {
            use std::time::Duration;
            let mut rx = rx;
            rx.run(infra, Duration::from_secs(10)).await;
        })
    };

    let bot = BotImpl::new(bot_config.bot_id, bot_config.bot_user_id);
    let wh = WebhookHandlerImpl::new();
    let app = bot_cnvtr::app::AppImpl::new_wrapped(bot, wh);
    let app = Arc::new(app);

    let router = make_router(&router_config.verification_token, infra, app)
        .layer(TraceLayer::new_for_http());
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!("listening on {} ...", addr);
    let serve = async move {
        router::serve(listener, router)
            .with_graceful_shutdown(bot_cnvtr::signal::signal())
            .await
    };

    tokio::select! {
        res = serve => {
            res?;
        }
        _ = handle => {}
    }
    Ok(())
}
