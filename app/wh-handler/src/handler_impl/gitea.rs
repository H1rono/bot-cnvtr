use http::HeaderMap;
use indoc::indoc;

use crate::Result;

pub(super) fn handle(_headers: HeaderMap, _payload: &str) -> Result<Option<String>> {
    let message = indoc! {r#"
        GiteaからWebhookが送信されました。
        実装は現在工事中です :construction:
    "#};
    // https://github.com/traPtitech/gitea/blob/8abe54a9d4db1fdce7c517dc500a51e77d1f2c16/services/webhook/deliver.go#L124-L138
    // https://github.com/traPtitech/gitea/blob/8abe54a9d4db1fdce7c517dc500a51e77d1f2c16/modules/webhook/type.go#L11-L33
    Ok(Some(message.to_string()))
}
