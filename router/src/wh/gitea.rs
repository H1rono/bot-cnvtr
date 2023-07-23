use axum::http::HeaderMap;
use indoc::indoc;
use serde_json::Value;

use crate::Result;

pub(super) fn handle(_headers: HeaderMap, _payload: Value) -> Result<String> {
    let message = indoc! {r#"
        GiteaからWebhookが送信されました。
        実装は現在工事中です :construction:
    "#};
    Ok(message.to_string())
}
