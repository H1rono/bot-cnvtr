use std::error::Error;
use std::ops::Deref;

use axum::{
    async_trait,
    body::{to_bytes, Body},
    extract::{FromRequest, State},
    http::{Request, StatusCode},
};
use traq_bot_http::Event;

use domain::{Repository, TraqClient};
use wh_handler::WebhookHandler;

use super::AppState;

#[derive(Debug, Clone)]
pub struct BotEvent(pub Event);

#[async_trait]
impl<Repo, C, WH, E1, E2> FromRequest<AppState<Repo, C, WH, E1, E2>> for BotEvent
where
    Repo: Repository<Error = E1>,
    C: TraqClient<Error = E2>,
    WH: WebhookHandler,
    usecases::Error: From<E1> + From<E2>,
{
    type Rejection = StatusCode;

    async fn from_request(
        req: Request<Body>,
        state: &AppState<Repo, C, WH, E1, E2>,
    ) -> Result<Self, Self::Rejection> {
        let parser = &state.parser;
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

pub(super) async fn event<Repo, C, WH, E1, E2>(
    State(st): State<AppState<Repo, C, WH, E1, E2>>,
    BotEvent(event): BotEvent,
) -> StatusCode
where
    Repo: Repository<Error = E1>,
    C: TraqClient<Error = E2>,
    WH: WebhookHandler,
    usecases::Error: From<E1> + From<E2>,
    E1: Send + Sync + 'static,
    E2: Send + Sync + 'static,
{
    let client = st.client.as_ref().lock().await;
    let repo = st.repo.as_ref().lock().await;
    match st
        .bot
        .handle_event(repo.deref(), client.deref(), event)
        .await
    {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(err) => {
            eprintln!("ERROR: {err}");
            eprintln!("{err:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
    // StatusCode::INTERNAL_SERVER_ERROR
}
