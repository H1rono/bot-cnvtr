use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    Json,
};
use serde_json::Value;
use uuid::Uuid;

use domain::Webhook;
use domain::{Repository, TraqClient};
use wh_handler::WebhookHandler;

use super::{AppState, Error, Result};

/// GET /wh/:id
pub(super) async fn get_wh<Repo, C, WH, E1, E2>(
    State(st): State<AppState<Repo, C, WH, E1, E2>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Webhook>>
where
    Repo: Repository<Error = E1>,
    C: TraqClient<Error = E2>,
    WH: WebhookHandler,
    usecases::Error: From<E1> + From<E2>,
{
    let repo = st.repo.as_ref().lock().await;
    repo.find_webhook(&id)
        .await
        .map_err(usecases::Error::from)?
        .ok_or(Error::NotFound)
        .map(Json)
}

/// POST /wh/:id/github
pub(super) async fn wh_github<Repo, C, WH, E1, E2>(
    State(st): State<AppState<Repo, C, WH, E1, E2>>,
    Path(id): Path<Uuid>,
    headers: HeaderMap,
    Json(payload): Json<Value>,
) -> Result<StatusCode>
where
    Repo: Repository<Error = E1>,
    C: TraqClient<Error = E2>,
    WH: WebhookHandler,
    usecases::Error: From<E1> + From<E2>,
{
    let client = st.client.as_ref().lock().await;
    let repo = st.repo.as_ref().lock().await;
    let webhook = repo
        .find_webhook(&id)
        .await
        .map_err(usecases::Error::from)?
        .ok_or(Error::NotFound)?;
    let message = st.wh.github_webhook(headers.iter(), payload)?;
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
pub(super) async fn wh_gitea<Repo, C, WH, E1, E2>(
    State(st): State<AppState<Repo, C, WH, E1, E2>>,
    Path(id): Path<Uuid>,
    headers: HeaderMap,
    Json(payload): Json<Value>,
) -> Result<StatusCode>
where
    Repo: Repository<Error = E1>,
    C: TraqClient<Error = E2>,
    WH: WebhookHandler,
    usecases::Error: From<E1> + From<E2>,
{
    let client = st.client.as_ref().lock().await;
    let repo = st.repo.as_ref().lock().await;
    let webhook = repo
        .find_webhook(&id)
        .await
        .map_err(usecases::Error::from)?
        .ok_or(Error::NotFound)?;
    let message = st.wh.gitea_webhook(headers.iter(), payload)?;
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
pub(super) async fn wh_clickup<Repo, C, WH, E1, E2>(
    State(st): State<AppState<Repo, C, WH, E1, E2>>,
    Path(id): Path<Uuid>,
    headers: HeaderMap,
    Json(payload): Json<Value>,
) -> Result<StatusCode>
where
    Repo: Repository<Error = E1>,
    C: TraqClient<Error = E2>,
    WH: WebhookHandler,
    usecases::Error: From<E1> + From<E2>,
{
    let client = st.client.as_ref().lock().await;
    let repo = st.repo.as_ref().lock().await;
    let webhook = repo
        .find_webhook(&id)
        .await
        .map_err(usecases::Error::from)?
        .ok_or(Error::NotFound)?;
    let message = st.wh.clickup_webhook(headers.iter(), payload)?;
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
