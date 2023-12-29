use indoc::formatdoc;
use serde_json::Value;

use super::utils::ValueExt;
use crate::Result;

pub(super) fn handle<'a, H, K, V>(_headers: H, payload: Value) -> Result<Option<String>>
where
    H: Iterator<Item = (&'a K, &'a V)>,
    K: AsRef<[u8]> + ?Sized + 'static,
    V: AsRef<[u8]> + ?Sized + 'static,
{
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
