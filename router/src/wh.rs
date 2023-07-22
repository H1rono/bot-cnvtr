use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    Json,
};
use indoc::{formatdoc, indoc};
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
    headers: HeaderMap,
    Json(payload): Json<Value>,
) -> Result<StatusCode> {
    let webhook = st.db.find_webhook(&id).await?.ok_or(Error::NotFound)?;
    let event_type = headers
        .get("X-GitHub-Event")
        .ok_or(Error::BadRequest)?
        .to_str()
        .map_err(|_| Error::BadRequest)?;
    let action = payload.get("action").and_then(Value::as_str);
    let ev_action = if let Some(act) = action {
        format!("{} {}", event_type, act)
    } else {
        event_type.to_string()
    };
    let repo_name = payload
        .get("repository")
        .ok_or(Error::BadRequest)?
        .get("full_name")
        .ok_or(Error::BadRequest)?
        .as_str()
        .ok_or(Error::BadRequest)?;
    let message = formatdoc! {
        r##"
            GitHubからWebhookが送信されました。
            リポジトリ: {}
            イベント: {}
            詳細は現在工事中です :construction:
        "##,
        repo_name,
        ev_action
    };
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
