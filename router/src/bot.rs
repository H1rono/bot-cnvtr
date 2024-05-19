use async_trait::async_trait;
use axum::{
    body::{to_bytes, Body},
    extract::{FromRequest, State},
};
use http::{Request, StatusCode};
use traq_bot_http::Event;

use usecases::{App, Bot};

use super::AppState;

#[derive(Debug, Clone)]
pub struct EventRequest(pub Event);

#[async_trait]
impl<S> FromRequest<S> for EventRequest
where
    S: AppState<Error = domain::Error>,
{
    type Rejection = StatusCode;

    #[tracing::instrument(skip_all, target = "router::bot::EventRequest::from_request")]
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
                tracing::error!("failed to parse bot event: {}", err);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }
}

#[tracing::instrument(
    skip_all,
    fields(event_kind = %event.kind()),
)]
pub(super) async fn event<S>(State(st): State<S>, EventRequest(event): EventRequest) -> StatusCode
where
    S: AppState<Error = domain::Error>,
{
    tracing::info!("POST traQ BOT event");
    match st.app().bot().handle_event(st.infra(), event).await {
        Ok(()) => StatusCode::NO_CONTENT,
        Err(err) => {
            tracing::error!(%err);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
