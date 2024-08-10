use std::{net::SocketAddr, sync::Arc};

use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

use bot::BotImpl;
use router::make_router;
use traq_client::ClientImpl;
use wh_handler::WebhookHandlerImpl;

use bot_cnvtr::{wrappers, ConfigComposite};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let env_filter = EnvFilter::try_from_env("CNVTR_LOG")
        .or_else(|_| EnvFilter::try_from_default_env())
        .unwrap_or_else(|_| "info".into());
    tracing_subscriber::fmt().with_env_filter(env_filter).init();

    let ConfigComposite {
        bot_config,
        router_config,
        client_config,
        repo_config,
        cron_config,
    } = ConfigComposite::from_env()
        .map_err(anyhow::Error::from)
        .or_else(|_| -> anyhow::Result<ConfigComposite> {
            dotenvy::from_filename_override(".env")?;
            Ok(ConfigComposite::from_env()?)
        })?;

    let client = ClientImpl::new(&client_config.bot_access_token);
    let repo_opt: repository::opt::Opt = repo_config.try_into()?;
    let repo = repo_opt.connect().await?;
    repo.migrate().await?;
    let (tx, rx) = cron::channel();
    let infra = wrappers::InfraImpl::new_wrapped(repo, client, tx);
    let infra = Arc::new(infra);

    // run notifier in background
    let cron_handle = {
        let infra = infra.clone();
        let period = cron_config.try_into()?;
        tokio::task::spawn(async move {
            let mut rx = rx;
            rx.run(infra, period).await;
        })
    };

    let bot = BotImpl::new(bot_config.name, bot_config.id, bot_config.user_id);
    let wh = WebhookHandlerImpl::new();
    let app = wrappers::AppImpl::new_wrapped(bot, wh);
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
        _ = cron_handle => unreachable!(),
    }
    Ok(())
}
