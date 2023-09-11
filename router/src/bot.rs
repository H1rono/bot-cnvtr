use std::error::Error;
use std::ops::Deref;

use axum::{
    async_trait,
    extract::{FromRequest, State},
    http::{Request, StatusCode},
};
use hyper::body::{to_bytes, Body};
use traq_bot_http::Event;

use repository::Database;

use super::AppState;

#[derive(Debug, Clone)]
pub struct BotEvent(pub Event);

#[async_trait]
impl<Db: Database> FromRequest<AppState<Db>, Body> for BotEvent {
    type Rejection = StatusCode;

    async fn from_request(
        req: Request<Body>,
        state: &AppState<Db>,
    ) -> Result<Self, Self::Rejection> {
        let parser = &state.parser;
        let (parts, body) = req.into_parts();
        let headers = parts.headers;
        let body = to_bytes(body).await.map_err(|_| StatusCode::BAD_REQUEST)?;
        match parser.parse(headers, &body) {
            Ok(event) => Ok(Self(event)),
            Err(err) => {
                eprintln!("ERROR: {err}");
                eprintln!("{err:?}");
                eprintln!("{:?}", err.source());
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }
}

pub(super) async fn event<Db: Database>(
    State(st): State<AppState<Db>>,
    BotEvent(event): BotEvent,
) -> StatusCode {
    let db = st.db.as_ref().lock().await;
    match st.bot.handle_event(db.deref(), event).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(err) => {
            eprintln!("ERROR: {err}");
            eprintln!("{err:?}");
            eprintln!("{:?}", err.source());
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
