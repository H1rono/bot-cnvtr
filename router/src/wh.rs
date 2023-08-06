use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    Json,
};
use serde_json::Value;
use uuid::Uuid;

use super::{AppState, Error, Result};
use model::{Database, Webhook};

mod clickup;
mod gitea;
mod github;
mod utils;

/// GET /wh/:id
pub(super) async fn get_wh(
    State(st): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Webhook>> {
    st.db
        .find_webhook(&id)
        .await?
        .ok_or(Error::NotFound)
        .map(Json)
}

/// POST /wh/:id/github
pub(super) async fn wh_github(
    State(st): State<AppState>,
    Path(id): Path<Uuid>,
    headers: HeaderMap,
    Json(payload): Json<Value>,
) -> Result<StatusCode> {
    let webhook = st.db.find_webhook(&id).await?.ok_or(Error::NotFound)?;
    let message = github::handle(headers, payload)?;
    if message.is_none() {
        return Ok(StatusCode::NO_CONTENT);
    }
    let message = message.unwrap();
    st.bot
        .send_message(&webhook.channel_id, message.trim(), false)
        .await
        .map_err(Error::from)?;
    Ok(StatusCode::NO_CONTENT)
}

/// POST /wh/:id/gitea
pub(super) async fn wh_gitea(
    State(st): State<AppState>,
    Path(id): Path<Uuid>,
    headers: HeaderMap,
    Json(payload): Json<Value>,
) -> Result<StatusCode> {
    let webhook = st.db.find_webhook(&id).await?.ok_or(Error::NotFound)?;
    let message = gitea::handle(headers, payload)?;
    st.bot
        .send_message(&webhook.channel_id, message.trim(), false)
        .await
        .map_err(Error::from)?;
    Ok(StatusCode::NOT_IMPLEMENTED)
}

/// POST /wh/:id/clickup
pub(super) async fn wh_clickup(
    State(st): State<AppState>,
    Path(id): Path<Uuid>,
    headers: HeaderMap,
    Json(payload): Json<Value>,
) -> Result<StatusCode> {
    let webhook = st.db.find_webhook(&id).await?.ok_or(Error::NotFound)?;
    let message = clickup::handle(headers, payload)?;
    st.bot
        .send_message(&webhook.channel_id, message.trim(), false)
        .await
        .map_err(Error::from)?;
    Ok(StatusCode::NOT_IMPLEMENTED)
}
