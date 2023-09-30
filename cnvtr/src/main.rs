use std::error::Error;
use std::net::SocketAddr;

use traq_bot_http::RequestParser;

use bot::Bot;
use config::Config;
use repository::RepositoryImpl;
use router::make_router;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let Config(bot_config, db_config) = Config::from_env()?;
    let parser = RequestParser::new(&bot_config.verification_token);
    let repo = RepositoryImpl::from_config(db_config).await?;
    repo.migrate().await?;
    let bot = Bot::from_config(bot_config);
    let app = make_router(repo, parser, bot);
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let server = axum::Server::bind(&addr).serve(app.into_make_service());
    println!("listening on {} ...", addr);
    server.await?;
    Ok(())
}
