use std::net::SocketAddr;

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
    let infra = bot_cnvtr::infra::InfraImpl::new_wrapped(repo, client);

    let bot = BotImpl::new(bot_config.bot_id, bot_config.bot_user_id);
    let wh = WebhookHandlerImpl::new();
    let app = bot_cnvtr::app::AppImpl::new_wrapped(bot, wh);

    let router = make_router(&router_config.verification_token, infra, app)
        .layer(TraceLayer::new_for_http());
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("listening on {} ...", addr);
    axum::serve(listener, router)
        .with_graceful_shutdown(bot_cnvtr::signal::signal())
        .await?;
    Ok(())
}
