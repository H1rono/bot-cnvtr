use std::sync::Arc;

use thiserror::Error as ThisError;

use axum::{
    body::Bytes,
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Router,
};

use traq_bot_http::RequestParser;

use super::{Bot, Database};

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
        .route("/", post(handler))
        .route("/wh/:id", get(wh::get_wh))
        .route("/wh/:id/github", post(wh::wh_github))
        .route("/wh/:id/gitea", post(wh::wh_gitea))
        .route("/wh/:id/clickup", post(wh::wh_clickup))
        .with_state(state)
}

async fn handler(State(st): State<AppState>, headers: HeaderMap, body: Bytes) -> StatusCode {
    match st.parser.parse(headers, &body) {
        Ok(event) => match st.bot.handle_event(st.db.as_ref(), event).await {
            Ok(_) => StatusCode::NO_CONTENT,
            Err(e) => {
                eprintln!("ERROR: {e}");
                StatusCode::INTERNAL_SERVER_ERROR
            }
        },
        Err(err) => {
            eprintln!("ERROR: {err}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

#[derive(Debug, ThisError)]
enum Error {
    #[error("not found")]
    NotFound,
    #[error("sql error")]
    SqlError(#[from] sqlx::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::NotFound => (StatusCode::NOT_FOUND, "Not Found").into_response(),
            Self::SqlError(e) => {
                eprintln!("sqlx error: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}

type Result<T, E = Error> = std::result::Result<T, E>;
