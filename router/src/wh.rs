use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    Json,
};
use serde_json::Value;
use uuid::Uuid;

use super::{AppState, Error, Result};
use repository::{AllRepository, Webhook, WebhookRepository};
use traq_client::Client;

mod clickup;
mod gitea;
mod github;
mod utils;

/// GET /wh/:id
pub(super) async fn get_wh<C: Client, Repo: AllRepository>(
    State(st): State<AppState<C, Repo>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Webhook>> {
    let repo = st.repo.as_ref().lock().await;
    repo.webhook_repository()
        .find(&id)
        .await?
        .ok_or(Error::NotFound)
        .map(Json)
}

/// POST /wh/:id/github
pub(super) async fn wh_github<C: Client, Repo: AllRepository>(
    State(st): State<AppState<C, Repo>>,
    Path(id): Path<Uuid>,
    headers: HeaderMap,
    Json(payload): Json<Value>,
) -> Result<StatusCode> {
    let client = st.client.as_ref().lock().await;
    let repo = st.repo.as_ref().lock().await;
    let webhook = repo
        .webhook_repository()
        .find(&id)
        .await?
        .ok_or(Error::NotFound)?;
    let message = github::handle(headers, payload)?;
    if message.is_none() {
        return Ok(StatusCode::NO_CONTENT);
    }
    let message = message.unwrap();
    client
        .send_message(&webhook.channel_id, message.trim(), false)
        .await
        .map_err(Error::from)?;
    Ok(StatusCode::NO_CONTENT)
}

/// POST /wh/:id/gitea
pub(super) async fn wh_gitea<C: Client, Repo: AllRepository>(
    State(st): State<AppState<C, Repo>>,
    Path(id): Path<Uuid>,
    headers: HeaderMap,
    Json(payload): Json<Value>,
) -> Result<StatusCode> {
    let client = st.client.as_ref().lock().await;
    let repo = st.repo.as_ref().lock().await;
    let webhook = repo
        .webhook_repository()
        .find(&id)
        .await?
        .ok_or(Error::NotFound)?;
    let message = gitea::handle(headers, payload)?;
    client
        .send_message(&webhook.channel_id, message.trim(), false)
        .await
        .map_err(Error::from)?;
    Ok(StatusCode::NOT_IMPLEMENTED)
}

/// POST /wh/:id/clickup
pub(super) async fn wh_clickup<C: Client, Db: AllRepository>(
    State(st): State<AppState<C, Db>>,
    Path(id): Path<Uuid>,
    headers: HeaderMap,
    Json(payload): Json<Value>,
) -> Result<StatusCode> {
    let client = st.client.as_ref().lock().await;
    let repo = st.repo.as_ref().lock().await;
    let webhook = repo
        .webhook_repository()
        .find(&id)
        .await?
        .ok_or(Error::NotFound)?;
    let message = clickup::handle(headers, payload)?;
    client
        .send_message(&webhook.channel_id, message.trim(), false)
        .await
        .map_err(Error::from)?;
    Ok(StatusCode::NOT_IMPLEMENTED)
}
