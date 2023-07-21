use std::sync::Arc;

use thiserror::Error as ThisError;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};

use traq_bot_http::RequestParser;

use super::{Bot, Database};

mod bot;
mod wh;

#[allow(dead_code)]
#[derive(Clone)]
struct AppState {
    pub db: Arc<Database>,
    pub parser: RequestParser,
    pub bot: Bot,
}

impl AppState {
    pub fn new(db: Database, parser: RequestParser, bot: Bot) -> Self {
        Self {
            db: Arc::new(db),
            parser,
            bot,
        }
    }
}

pub fn make_router(db: Database, parser: RequestParser, bot: Bot) -> Router {
    let state = AppState::new(db, parser, bot);
    Router::new()
        .route("/bot", post(bot::event))
        .route("/wh/:id", get(wh::get_wh))
        .route("/wh/:id/github", post(wh::wh_github))
        .route("/wh/:id/gitea", post(wh::wh_gitea))
        .route("/wh/:id/clickup", post(wh::wh_clickup))
        .with_state(state)
}

#[derive(Debug, ThisError)]
enum Error {
    #[error("not found")]
    NotFound,
    #[error("sqlx error")]
    Sqlx(#[from] sqlx::Error),
    #[error("processing error")]
    Process(#[from] crate::bot::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::NotFound => (StatusCode::NOT_FOUND, "Not Found").into_response(),
            Self::Sqlx(e) => {
                eprintln!("sqlx error: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
            Self::Process(e) => {
                eprintln!("processing error: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}

type Result<T, E = Error> = std::result::Result<T, E>;
