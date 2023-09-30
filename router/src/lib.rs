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

mod bot;
mod error;
mod wh;

use error::{Error, Result};

struct AppState<Repo: AllRepository> {
    pub db: Arc<Mutex<Repo>>,
    pub parser: RequestParser,
    pub bot: Bot,
}

impl<Repo: AllRepository> Clone for AppState<Repo> {
    fn clone(&self) -> Self {
        Self {
            db: self.db.clone(),
            parser: self.parser.clone(),
            bot: self.bot.clone(),
        }
    }
}

impl<Repo: AllRepository> AppState<Repo> {
    pub fn new(db: Repo, parser: RequestParser, bot: Bot) -> Self {
        Self {
            db: Arc::new(Mutex::new(db)),
            parser,
            bot,
        }
    }
}

impl<Repo: AllRepository> AsRef<AppState<Repo>> for State<AppState<Repo>> {
    fn as_ref(&self) -> &AppState<Repo> {
        &self.0
    }
}

pub fn make_router<Repo: AllRepository>(db: Repo, parser: RequestParser, bot: Bot) -> Router {
    let state = AppState::new(db, parser, bot);
    Router::new()
        .route("/bot", post(bot::event::<Repo>))
        .route("/wh/:id", get(wh::get_wh::<Repo>))
        .route("/wh/:id/github", post(wh::wh_github::<Repo>))
        .route("/wh/:id/gitea", post(wh::wh_gitea::<Repo>))
        .route("/wh/:id/clickup", post(wh::wh_clickup::<Repo>))
        .with_state(state)
}
