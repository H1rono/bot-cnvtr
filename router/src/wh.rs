use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use indoc::indoc;
use serde_json::Value;
use uuid::Uuid;

use super::{AppState, Error, Result};
use model::Webhook;

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
    Json(_payload): Json<Value>,
) -> Result<StatusCode> {
    let webhook = st.db.find_webhook(&id).await?.ok_or(Error::NotFound)?;
    let message = indoc! {
        r##"
            GitHubからWebhookが送信されました。
            実装は現在工事中です :construction:
        "##
    };
    st.bot
        .send_message(&webhook.channel_id, message, false)
        .await
        .map_err(Error::from)?;
    Ok(StatusCode::NOT_IMPLEMENTED)
}

/// POST /wh/:id/gitea
pub(super) async fn wh_gitea(
    State(st): State<AppState>,
    Path(id): Path<Uuid>,
    Json(_payload): Json<Value>,
) -> Result<StatusCode> {
    let webhook = st.db.find_webhook(&id).await?.ok_or(Error::NotFound)?;
    let message = indoc! {
        r##"
            GiteaからWebhookが送信されました。
            実装は現在工事中です :construction:
        "##
    };
    st.bot
        .send_message(&webhook.channel_id, message, false)
        .await
        .map_err(Error::from)?;
    Ok(StatusCode::NOT_IMPLEMENTED)
}

/// POST /wh/:id/clickup
pub(super) async fn wh_clickup(
    State(st): State<AppState>,
    Path(id): Path<Uuid>,
    Json(_payload): Json<Value>,
) -> Result<StatusCode> {
    let webhook = st.db.find_webhook(&id).await?.ok_or(Error::NotFound)?;
    let message = indoc! {
        r##"
            ClickUpからWebhookが送信されました。
            実装は現在工事中です :construction:
        "##
    };
    st.bot
        .send_message(&webhook.channel_id, message, false)
        .await
        .map_err(Error::from)?;
    Ok(StatusCode::NOT_IMPLEMENTED)
}
