use std::str::from_utf8;

use http::HeaderMap;
use indoc::formatdoc;
use serde_json::Value;
use teahook_rs as th;

use super::utils::extract_header_value;
use crate::{Error, Result};

pub(super) fn handle(headers: HeaderMap, payload: &str) -> Result<Option<String>> {
    use serde_json::from_str;
    // https://github.com/traPtitech/gitea/blob/8abe54a9d4db1fdce7c517dc500a51e77d1f2c16/services/webhook/deliver.go#L124-L138
    // https://github.com/traPtitech/gitea/blob/8abe54a9d4db1fdce7c517dc500a51e77d1f2c16/modules/webhook/type.go#L11-L33
    let event_type = extract_header_value(&headers, "X-Gitea-Event")
        .and_then(|v| from_utf8(v).map_err(|_| Error::WrongType))?;
    let message = match event_type {
        "create" => Some(create(from_str(payload)?)),
        _ => default(event_type, from_str(payload)?),
    };
    let Some(message) = message else {
        return Ok(None);
    };
    Ok(Some(message))
}

/// X-Gitea-Event: create
fn create(payload: th::CreatePayload) -> String {
    let th::CreatePayload {
        r#ref,
        ref_type,
        repo,
        sender,
        ..
    } = &payload;
    formatdoc! {
        r##"
            [{}] {} `{}` was created by {}.
        "##,
        repo_str(repo), ref_type, r#ref, user_str(sender)
    }
}

/// X-Gitea-Event: *
fn default(event_type: &str, _payload: Value) -> Option<String> {
    Some(formatdoc! {
        r#"
            GiteaからWebhookが送信されました。 イベント: {}
            実装は現在工事中です :constructon:
        "#,
        event_type
    })
}

fn repo_str(repo: &th::Repository) -> String {
    format!("[{}]({})", repo.full_name, repo.html_url)
}

fn user_str(user: &th::User) -> String {
    format!("[{}]({})", user.user_name, user.avatar_url)
}
