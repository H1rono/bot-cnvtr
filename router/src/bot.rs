use std::error::Error;

use axum::{
    async_trait,
    body::{to_bytes, Body},
    extract::{FromRequest, State},
    http::{Request, StatusCode},
};
use traq_bot_http::Event;

use usecases::Bot;

use super::AppState;

#[derive(Debug, Clone)]
pub struct BotEvent(pub Event);

#[async_trait]
impl<S> FromRequest<S> for BotEvent
where
    S: AppState<Error = domain::Error>,
{
    type Rejection = StatusCode;

    async fn from_request(req: Request<Body>, state: &S) -> Result<Self, Self::Rejection> {
        let parser = state.parser();
        let (parts, body) = req.into_parts();
        let headers = parts.headers;
        let body = to_bytes(body, usize::MAX)
            .await
            .map_err(|_| StatusCode::BAD_REQUEST)?;
        match parser.parse(headers.iter(), &body) {
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

pub(super) async fn event<S>(State(st): State<S>, BotEvent(event): BotEvent) -> StatusCode
where
    S: AppState<Error = domain::Error>,
{
    match st.bot().handle_event(st.infra(), event).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(err) => {
            eprintln!("ERROR: {err}");
            eprintln!("{err:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
