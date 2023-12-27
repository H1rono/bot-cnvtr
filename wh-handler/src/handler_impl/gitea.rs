use indoc::indoc;
use serde_json::Value;

use crate::Result;

pub(super) fn handle<'a, H, K, V>(_headers: H, _payload: Value) -> Result<Option<String>>
where
    H: Iterator<Item = (&'a K, &'a V)>,
    K: AsRef<[u8]> + ?Sized + 'static,
    V: AsRef<[u8]> + ?Sized + 'static,
{
    let message = indoc! {r#"
        GiteaからWebhookが送信されました。
        実装は現在工事中です :construction:
    "#};
    Ok(Some(message.to_string()))
}
