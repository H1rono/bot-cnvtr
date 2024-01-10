use http::HeaderMap;
use indoc::indoc;

use crate::Result;

pub(super) fn handle(_headers: HeaderMap, _payload: &str) -> Result<Option<String>> {
    let message = indoc! {r#"
        GiteaからWebhookが送信されました。
        実装は現在工事中です :construction:
    "#};
    Ok(Some(message.to_string()))
}
