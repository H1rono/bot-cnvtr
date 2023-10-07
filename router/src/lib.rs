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
use wh_handler::WebhookHandler;

mod bot;
mod config;
mod error;
mod wh;

pub use config::Config;
use error::{Error, Result};

struct AppState<C: Client, Repo: AllRepository, WH: WebhookHandler> {
    pub client: Arc<Mutex<C>>,
    pub repo: Arc<Mutex<Repo>>,
    pub wh: WH,
    pub parser: RequestParser,
    pub bot: Bot,
}

impl<C: Client, Repo: AllRepository, WH: WebhookHandler> Clone for AppState<C, Repo, WH> {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            repo: self.repo.clone(),
            wh: self.wh.clone(),
            parser: self.parser.clone(),
            bot: self.bot.clone(),
        }
    }
}

impl<C: Client, Repo: AllRepository, WH: WebhookHandler> AppState<C, Repo, WH> {
    pub fn new(client: C, repo: Repo, wh: WH, parser: RequestParser, bot: Bot) -> Self {
        Self {
            client: Arc::new(Mutex::new(client)),
            repo: Arc::new(Mutex::new(repo)),
            wh,
            parser,
            bot,
        }
    }
}

impl<C: Client, Repo: AllRepository, WH: WebhookHandler> AsRef<AppState<C, Repo, WH>>
    for State<AppState<C, Repo, WH>>
{
    fn as_ref(&self) -> &AppState<C, Repo, WH> {
        &self.0
    }
}

pub fn make_router<C: Client, Repo: AllRepository, WH: WebhookHandler>(
    config: Config,
    client: C,
    wh: WH,
    repo: Repo,
    bot: Bot,
) -> Router {
    let parser = config.into();
    let state = AppState::new(client, repo, wh, parser, bot);
    Router::new()
        .route("/bot", post(bot::event::<C, Repo, WH>))
        .route("/wh/:id", get(wh::get_wh::<C, Repo, WH>))
        .route("/wh/:id/github", post(wh::wh_github::<C, Repo, WH>))
        .route("/wh/:id/gitea", post(wh::wh_gitea::<C, Repo, WH>))
        .route("/wh/:id/clickup", post(wh::wh_clickup::<C, Repo, WH>))
        .with_state(state)
}
