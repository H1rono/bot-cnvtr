use std::error::Error;
use std::net::SocketAddr;

use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

use ::bot::BotImpl;
use ::traq_client::ClientImpl;
use ::wh_handler::WebhookHandlerImpl;
use repository::RepositoryImpl;
use router::make_router;

pub mod app;
pub mod bot;
pub mod config;
pub mod infra;
pub mod repo;
pub mod traq_client;
pub mod wh_handler;

use config::ConfigComposite;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or("info".into()))
        .init();
    let ConfigComposite {
        bot_config,
        router_config,
        client_config,
        repo_config,
    } = ConfigComposite::from_env().or_else(|_| -> Result<_, Box<dyn Error>> {
        dotenvy::from_filename_override(".env")?;
        Ok(ConfigComposite::from_env()?)
    })?;
    let client = ClientImpl::from_config(client_config);
    let repo = RepositoryImpl::from_config(repo_config).await?;
    repo.migrate().await?;
    let infra = infra::InfraImpl::new_wrapped(repo, client);
    let bot = BotImpl::from_config(bot_config);
    let bot = bot::BotWrapper::new(bot);
    let wh = WebhookHandlerImpl::new();
    let wh = wh_handler::WHandlerWrapper::new(wh);
    let app = app::AppImpl(bot, wh);
    let router = make_router(router_config, infra, app).layer(TraceLayer::new_for_http());
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("listening on {} ...", addr);
    axum::serve(listener, router).await?;
    Ok(())
}
