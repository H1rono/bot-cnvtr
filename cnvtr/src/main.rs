use std::error::Error;
use std::net::SocketAddr;

use repository::RepositoryImpl;
use router::make_router;
use traq_client::ClientImpl;
use usecases::BotImpl;
use wh_handler::WebhookHandlerImpl;

pub mod config;
pub mod infra;

use config::ConfigComposite;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let ConfigComposite {
        usecases_config,
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
    let infra = infra::InfraImpl(repo, client);
    let usecases = BotImpl::from_config(usecases_config);
    let wh = WebhookHandlerImpl::new();
    let app = make_router(router_config, infra, wh, usecases);
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("listening on {} ...", addr);
    axum::serve(listener, app).await?;
    Ok(())
}
