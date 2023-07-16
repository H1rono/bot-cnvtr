use std::sync::Arc;

use axum::{
    body::Bytes,
    extract::State,
    http::{HeaderMap, StatusCode},
    routing::post,
    Router,
};

use traq_bot_http::RequestParser;

use super::{Bot, Database};

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
    Router::new().route("/", post(handler)).with_state(state)
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
