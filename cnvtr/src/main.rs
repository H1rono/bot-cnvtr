use std::error::Error;
use std::net::SocketAddr;

use bot::Bot;
use repository::RepositoryImpl;
use router::make_router;
use traq_client::ClientImpl;
use wh_handler::WebhookHandlerImpl;

mod config;

use config::ConfigComposite;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
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
    let bot = Bot::from_config(bot_config);
    let wh = WebhookHandlerImpl::new();
    let app = make_router(router_config, client, wh, repo, bot);
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let server = axum::Server::bind(&addr).serve(app.into_make_service());
    println!("listening on {} ...", addr);
    server.await?;
    Ok(())
}
