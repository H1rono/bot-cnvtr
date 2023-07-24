use axum::http::HeaderMap;
use indoc::formatdoc;
use serde_json::Value;

use crate::{Error, Result};

pub(super) fn handle(_headers: HeaderMap, payload: Value) -> Result<String> {
    let event = payload
        .get("event")
        .ok_or(Error::BadRequest)?
        .as_str()
        .ok_or(Error::BadRequest)?;
    let message = formatdoc! {
        r#"
            ClickUpからWebhookが送信されました。
            イベント: {}
            実装は現在工事中です :construction:
        "#,
        event
    };
    Ok(message)
}
