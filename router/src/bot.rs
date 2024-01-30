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

    #[tracing::instrument(skip_all, target = "router::bot::BotEvent::from_request")]
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
pub(super) async fn event<S>(State(st): State<S>, BotEvent(event): BotEvent) -> StatusCode
where
    S: AppState<Error = domain::Error>,
{
    tracing::info!("POST traQ BOT event");
    match st.bot().handle_event(st.infra(), event).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(err) => {
            tracing::error!(%err);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
