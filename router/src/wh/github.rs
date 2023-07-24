use axum::http::HeaderMap;
use indoc::formatdoc;
use serde_json::Value;

use super::utils::ValueExt;
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
        .get_or_err("repository")?
        .get_or_err("full_name")?
        .as_str_or_err()?;
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
