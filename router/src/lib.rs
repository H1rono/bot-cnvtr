use std::sync::Arc;

use axum::{
    extract::State,
    routing::{get, post},
    Router,
};

use traq_bot_http::RequestParser;

use ::bot::Bot;
use model::DatabaseImpl;

mod bot;
mod error;
mod wh;

use error::{Error, Result};

#[derive(Clone)]
struct AppState {
    pub db: Arc<DatabaseImpl>,
    pub parser: RequestParser,
    pub bot: Bot,
}

impl AppState {
    pub fn new(db: DatabaseImpl, parser: RequestParser, bot: Bot) -> Self {
        Self {
            db: Arc::new(db),
            parser,
            bot,
        }
    }
}

impl AsRef<AppState> for State<AppState> {
    fn as_ref(&self) -> &AppState {
        &self.0
    }
}

pub fn make_router(db: DatabaseImpl, parser: RequestParser, bot: Bot) -> Router {
    let state = AppState::new(db, parser, bot);
    Router::new()
        .route("/bot", post(bot::event))
        .route("/wh/:id", get(wh::get_wh))
        .route("/wh/:id/github", post(wh::wh_github))
        .route("/wh/:id/gitea", post(wh::wh_gitea))
        .route("/wh/:id/clickup", post(wh::wh_clickup))
        .with_state(state)
}
