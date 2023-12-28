use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    Json,
};
use serde_json::Value;
use uuid::Uuid;

use domain::Webhook;
use domain::{Infra, Repository, TraqClient};
use usecases::WebhookHandler;

use super::{AppState, Error, Result};

/// GET /wh/:id
pub(super) async fn get_wh<I, WH, E1, E2, E3>(
    State(st): State<AppState<I, WH, E1, E2, E3>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Webhook>>
where
    I: Infra,
    I::Repo: Repository<Error = E1>,
    I::TClient: TraqClient<Error = E2>,
    WH: WebhookHandler<Error = E3>,
    usecases::Error: From<E1> + From<E2> + From<E3>,
{
    let repo = st.infra.repo();
    repo.find_webhook(&id)
        .await
        .map_err(usecases::Error::from)?
        .ok_or(Error::NotFound)
        .map(Json)
}

/// POST /wh/:id/github
pub(super) async fn wh_github<I, WH, E1, E2, E3>(
    State(st): State<AppState<I, WH, E1, E2, E3>>,
    Path(id): Path<Uuid>,
    headers: HeaderMap,
    Json(payload): Json<Value>,
) -> Result<StatusCode>
where
    I: Infra,
    I::Repo: Repository<Error = E1>,
    I::TClient: TraqClient<Error = E2>,
    WH: WebhookHandler<Error = E3>,
    usecases::Error: From<E1> + From<E2> + From<E3>,
{
    let client = st.infra.traq_client();
    let repo = st.infra.repo();
    let webhook = repo
        .find_webhook(&id)
        .await
        .map_err(usecases::Error::from)?
        .ok_or(Error::NotFound)?;
    let message = st
        .wh
        .github_webhook(headers.iter(), payload)
        .map_err(usecases::Error::from)?;
    if message.is_none() {
        return Ok(StatusCode::NO_CONTENT);
    }
    let message = message.unwrap();
    client
        .send_message(&webhook.channel_id, message.trim(), false)
        .await
        .map_err(usecases::Error::from)?;
    Ok(StatusCode::NO_CONTENT)
}

/// POST /wh/:id/gitea
pub(super) async fn wh_gitea<I, WH, E1, E2, E3>(
    State(st): State<AppState<I, WH, E1, E2, E3>>,
    Path(id): Path<Uuid>,
    headers: HeaderMap,
    Json(payload): Json<Value>,
) -> Result<StatusCode>
where
    I: Infra,
    I::Repo: Repository<Error = E1>,
    I::TClient: TraqClient<Error = E2>,
    WH: WebhookHandler<Error = E3>,
    usecases::Error: From<E1> + From<E2> + From<E3>,
{
    let client = st.infra.traq_client();
    let repo = st.infra.repo();
    let webhook = repo
        .find_webhook(&id)
        .await
        .map_err(usecases::Error::from)?
        .ok_or(Error::NotFound)?;
    let message = st
        .wh
        .gitea_webhook(headers.iter(), payload)
        .map_err(usecases::Error::from)?;
    if message.is_none() {
        return Ok(StatusCode::NO_CONTENT);
    }
    let message = message.unwrap();
    client
        .send_message(&webhook.channel_id, message.trim(), false)
        .await
        .map_err(usecases::Error::from)?;
    Ok(StatusCode::NOT_IMPLEMENTED)
}

/// POST /wh/:id/clickup
pub(super) async fn wh_clickup<I, WH, E1, E2, E3>(
    State(st): State<AppState<I, WH, E1, E2, E3>>,
    Path(id): Path<Uuid>,
    headers: HeaderMap,
    Json(payload): Json<Value>,
) -> Result<StatusCode>
where
    I: Infra,
    I::Repo: Repository<Error = E1>,
    I::TClient: TraqClient<Error = E2>,
    WH: WebhookHandler<Error = E3>,
    usecases::Error: From<E1> + From<E2> + From<E3>,
{
    let client = st.infra.traq_client();
    let repo = st.infra.repo();
    let webhook = repo
        .find_webhook(&id)
        .await
        .map_err(usecases::Error::from)?
        .ok_or(Error::NotFound)?;
    let message = st
        .wh
        .clickup_webhook(headers.iter(), payload)
        .map_err(usecases::Error::from)?;
    if message.is_none() {
        return Ok(StatusCode::NO_CONTENT);
    }
    let message = message.unwrap();
    client
        .send_message(&webhook.channel_id, message.trim(), false)
        .await
        .map_err(usecases::Error::from)?;
    Ok(StatusCode::NOT_IMPLEMENTED)
}
