use std::sync::Arc;

use axum::{
    extract::State,
    routing::{get, post},
    Router,
};
use tokio::sync::Mutex;

use traq_bot_http::RequestParser;

use ::bot::Bot;
use model::Database;

mod bot;
mod error;
mod wh;

use error::{Error, Result};

struct AppState<Db: Database> {
    pub db: Arc<Mutex<Db>>,
    pub parser: RequestParser,
    pub bot: Bot,
}

impl<Db: Database> AppState<Db> {
    pub fn new(db: Db, parser: RequestParser, bot: Bot) -> Self {
        Self {
            db: Arc::new(Mutex::new(db)),
            parser,
            bot,
        }
    }
}

impl<Db: Database> Clone for AppState<Db> {
    fn clone(&self) -> Self {
        Self {
            db: self.db.clone(),
            parser: self.parser.clone(),
            bot: self.bot.clone(),
        }
    }
}

impl<Db: Database> AsRef<AppState<Db>> for State<AppState<Db>> {
    fn as_ref(&self) -> &AppState<Db> {
        &self.0
    }
}

pub fn make_router<Db: Database>(db: Db, parser: RequestParser, bot: Bot) -> Router {
    let state = AppState::new(db, parser, bot);
    Router::new()
        .route("/bot", post(bot::event::<Db>))
        .route("/wh/:id", get(wh::get_wh::<Db>))
        .route("/wh/:id/github", post(wh::wh_github::<Db>))
        .route("/wh/:id/gitea", post(wh::wh_gitea::<Db>))
        .route("/wh/:id/clickup", post(wh::wh_clickup::<Db>))
        .with_state(state)
}
