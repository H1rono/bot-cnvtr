use std::error::Error;

use axum::{
    async_trait,
    body::{to_bytes, Body},
    extract::{FromRequest, State},
    http::{Request, StatusCode},
};
use traq_bot_http::Event;

use domain::{Infra, Repository, TraqClient};
use usecases::WebhookHandler;

use super::AppState;

#[derive(Debug, Clone)]
pub struct BotEvent(pub Event);

#[async_trait]
impl<I, WH, E1, E2, E3> FromRequest<AppState<I, WH, E1, E2, E3>> for BotEvent
where
    I: Infra,
    I::Repo: Repository<Error = E1>,
    I::TClient: TraqClient<Error = E2>,
    WH: WebhookHandler<Error = E3>,
    usecases::Error: From<E1> + From<E2> + From<E3>,
{
    type Rejection = StatusCode;

    async fn from_request(
        req: Request<Body>,
        state: &AppState<I, WH, E1, E2, E3>,
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

pub(super) async fn event<I, WH, E1, E2, E3>(
    State(st): State<AppState<I, WH, E1, E2, E3>>,
    BotEvent(event): BotEvent,
) -> StatusCode
where
    I: Infra,
    I::Repo: Repository<Error = E1>,
    I::TClient: TraqClient<Error = E2>,
    WH: WebhookHandler<Error = E3>,
    usecases::Error: From<E1> + From<E2> + From<E3>,
{
    let client = st.infra.traq_client();
    let repo = st.infra.repo();
    match st.bot.handle_event(repo, client, event).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(err) => {
            eprintln!("ERROR: {err}");
            eprintln!("{err:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
    // StatusCode::INTERNAL_SERVER_ERROR
}
