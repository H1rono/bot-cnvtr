use std::error::Error;
use std::net::SocketAddr;

use traq_bot_http::RequestParser;

use bot::Bot;
use repository::RepositoryImpl;
use router::make_router;
use traq_client::ClientImpl;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (bot_config, client_config, db_config) = load_config()?;
    let parser = RequestParser::new(&bot_config.verification_token);
    let client = ClientImpl::from_config(client_config);
    let repo = RepositoryImpl::from_config(db_config).await?;
    repo.migrate().await?;
    let bot = Bot::from_config(bot_config);
    let app = make_router(client, repo, parser, bot);
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let server = axum::Server::bind(&addr).serve(app.into_make_service());
    println!("listening on {} ...", addr);
    server.await?;
    Ok(())
}

fn load_config() -> Result<(bot::Config, traq_client::Config, repository::Config), Box<dyn Error>> {
    let bot_config = bot::Config::from_env();
    let client_config = traq_client::Config::from_env();
    let db_config = repository::Config::from_env();
    dotenvy::from_filename_override(".env")?;
    let bot_config = bot_config.or_else(|_| bot::Config::from_env())?;
    let client_config = client_config.or_else(|_| traq_client::Config::from_env())?;
    let db_config = db_config.or_else(|_| repository::Config::from_env())?;
    Ok((bot_config, client_config, db_config))
}
