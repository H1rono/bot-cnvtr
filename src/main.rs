use bot_cnvtr as lib;

use std::sync::Arc;

use tracing_subscriber::EnvFilter;

use lib::{wrappers, ConfigComposite};

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
        .or_else(|_| -> anyhow::Result<_> {
            dotenvy::from_filename_override(".env")?;
            Ok(ConfigComposite::from_env()?)
        })?;

    let client = traq_client::ClientImpl::new(&client_config.bot_access_token);
    let repo_opt: repository::opt::Opt = repo_config.try_into()?;
    let repo = repo_opt.connect().await?;
    repo.migrate().await?;
    let (tx, rx) = cron::channel();
    let infra = wrappers::InfraImpl::new_wrapped(repo, client, tx);
    let infra = Arc::new(infra);

    // run notifier in background
    let cron_handle = {
        let infra = Arc::clone(&infra);
        let period = cron_config.try_into()?;
        tokio::task::spawn(async move {
            rx.run(infra, period).await;
        })
    };

    let bot = bot::BotImpl::new(bot_config.name, bot_config.id, bot_config.user_id);
    let wh = wh_handler::WebhookHandlerImpl::new();
    let app = wrappers::AppImpl::new_wrapped(bot, wh);
    let app = Arc::new(app);

    let router = router::make_router(&router_config.verification_token, infra, app)
        .layer(tower_http::trace::TraceLayer::new_for_http());
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!("listening on {addr} ...");
    let shutdown_signal = bot_cnvtr::signal::signal();
    let serve = router::serve(listener, router).with_graceful_shutdown(shutdown_signal);

    tokio::select! {
        res = serve => {
            res?;
        }
        _ = cron_handle => unreachable!(),
    }
    Ok(())
}
