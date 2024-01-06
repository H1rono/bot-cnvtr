use http::HeaderMap;
use indoc::indoc;
use serde_json::Value;

use crate::Result;

pub(super) fn handle(_headers: HeaderMap, _payload: Value) -> Result<Option<String>> {
    let message = indoc! {r#"
        GiteaからWebhookが送信されました。
        実装は現在工事中です :construction:
    "#};
    Ok(Some(message.to_string()))
}
