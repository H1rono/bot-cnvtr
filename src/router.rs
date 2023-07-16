use std::sync::Arc;

use axum::{
    body::Bytes,
    extract::State,
    http::{HeaderMap, StatusCode},
    routing::post,
    Router,
};

use traq_bot_http::{Event, RequestParser};

use super::Database;

#[allow(dead_code)]
#[derive(Clone)]
struct AppState {
    pub db: Arc<Database>,
    pub parser: RequestParser,
}

impl AppState {
    pub fn new(db: Database, parser: RequestParser) -> Self {
        Self {
            db: Arc::new(db),
            parser,
        }
    }
}

pub fn make_router(db: Database, parser: RequestParser) -> Router {
    let state = AppState::new(db, parser);
    Router::new().route("/", post(handler)).with_state(state)
}

async fn handler(State(st): State<AppState>, headers: HeaderMap, body: Bytes) -> StatusCode {
    use Event::*;
    match st.parser.parse(headers, &body) {
        Ok(Joined(payload)) => {
            println!("チャンネル {} に参加しました。", payload.channel.name);
            StatusCode::NO_CONTENT
        }
        Ok(Left(payload)) => {
            println!("チャンネル {} から退出しました。", payload.channel.name);
            StatusCode::NO_CONTENT
        }
        Ok(MessageCreated(payload)) => {
            print!(
                "{}さんがメッセージを投稿しました。\n内容: {}\n",
                payload.message.user.display_name, payload.message.text
            );
            StatusCode::NO_CONTENT
        }
        Ok(_) => StatusCode::NO_CONTENT,
        Err(err) => {
            eprintln!("ERROR: {err}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
