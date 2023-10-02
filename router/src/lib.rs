use std::sync::Arc;

use axum::{
    extract::State,
    routing::{get, post},
    Router,
};
use tokio::sync::Mutex;

use traq_bot_http::RequestParser;

use ::bot::Bot;
use repository::AllRepository;
use traq_client::Client;

mod bot;
mod config;
mod error;
mod wh;

pub use config::Config;
use error::{Error, Result};

struct AppState<C: Client, Repo: AllRepository> {
    pub client: Arc<Mutex<C>>,
    pub repo: Arc<Mutex<Repo>>,
    pub parser: RequestParser,
    pub bot: Bot,
}

impl<C: Client, Repo: AllRepository> Clone for AppState<C, Repo> {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            repo: self.repo.clone(),
            parser: self.parser.clone(),
            bot: self.bot.clone(),
        }
    }
}

impl<C: Client, Repo: AllRepository> AppState<C, Repo> {
    pub fn new(client: C, repo: Repo, parser: RequestParser, bot: Bot) -> Self {
        Self {
            client: Arc::new(Mutex::new(client)),
            repo: Arc::new(Mutex::new(repo)),
            parser,
            bot,
        }
    }
}

impl<C: Client, Repo: AllRepository> AsRef<AppState<C, Repo>> for State<AppState<C, Repo>> {
    fn as_ref(&self) -> &AppState<C, Repo> {
        &self.0
    }
}

pub fn make_router<C: Client, Repo: AllRepository>(
    config: Config,
    client: C,
    repo: Repo,
    bot: Bot,
) -> Router {
    let parser = config.into();
    let state = AppState::new(client, repo, parser, bot);
    Router::new()
        .route("/bot", post(bot::event::<C, Repo>))
        .route("/wh/:id", get(wh::get_wh::<C, Repo>))
        .route("/wh/:id/github", post(wh::wh_github::<C, Repo>))
        .route("/wh/:id/gitea", post(wh::wh_gitea::<C, Repo>))
        .route("/wh/:id/clickup", post(wh::wh_clickup::<C, Repo>))
        .with_state(state)
}
