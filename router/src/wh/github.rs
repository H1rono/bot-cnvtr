use axum::http::HeaderMap;
use indoc::formatdoc;
use serde_json::Value;

use crate::{Error, Result};

pub(super) fn handle(headers: HeaderMap, payload: Value) -> Result<String> {
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
    Ok(message)
}
