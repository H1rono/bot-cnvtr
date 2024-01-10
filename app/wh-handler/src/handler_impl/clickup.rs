use http::HeaderMap;
use indoc::formatdoc;
use serde_json::Value;

use super::utils::ValueExt;
use crate::Result;

pub(super) fn handle(_headers: HeaderMap, payload: &str) -> Result<Option<String>> {
    let payload: Value = serde_json::from_str(payload)?;
    let event = payload.get_or_err("event")?.as_str_or_err()?;
    let message = formatdoc! {
        r#"
            ClickUpからWebhookが送信されました。
            イベント: {}
            実装は現在工事中です :construction:
        "#,
        event
    };
    Ok(Some(message))
}
