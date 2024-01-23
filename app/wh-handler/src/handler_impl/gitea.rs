use std::{ops::Deref, str::from_utf8};

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
        "delete" => Some(delete(from_str(payload)?)),
        "fork" => Some(fork(from_str(payload)?)),
        "push" => Some(push(from_str(payload)?)),
        "issues" => Some(issues(from_str(payload)?)),
        "pull_request" => Some(pull_request(from_str(payload)?)),
        "issue_assign"
        | "issue_label"
        | "issue_milestone"
        | "issue_comment"
        | "pull_request_assign"
        | "pull_request_label"
        | "pull_request_milestone"
        | "pull_request_comment"
        | "pull_request_review_approved"
        | "pull_request_review_rejected"
        | "pull_request_review_comment"
        | "pull_request_sync"
        | "pull_request_review_request"
        | "wiki"
        | "repository"
        | "release"
        | "package" => default(event_type, from_str(payload)?),
        _ => {
            // TODO: tracing
            eprintln!(
                "received unexpected header: `X-Gitea-Event: {}`",
                event_type
            );
            return Err(Error::WrongType);
        }
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
            [{}] {} `{}` was created by {}
        "##,
        repo_str(repo), ref_type, r#ref, user_str(sender)
    }
}

/// X-Gitea-Event: delete
fn delete(payload: th::DeletePayload) -> String {
    let th::DeletePayload {
        r#ref,
        ref_type,
        repo,
        sender,
        ..
    } = &payload;
    formatdoc! {
        r#"
            [{}] {} `{}` was deleted by {}
        "#,
        repo_str(repo), ref_type, r#ref, user_str(sender)
    }
}

/// X-Gitea-Event: fork
fn fork(payload: th::ForkPayload) -> String {
    let th::ForkPayload {
        forkee,
        repo,
        sender,
    } = &payload;
    formatdoc! {
        r#"
            [{}] forked to {} by {}
        "#,
        repo_str(repo), repo_str(forkee), user_str(sender)
    }
}

/// X-Gitea-Event: push
fn push(payload: th::PushPayload) -> String {
    let th::PushPayload {
        r#ref,
        commits,
        repo,
        sender,
        ..
    } = &payload;
    let commit_count = commits.len();
    let commit_unit = if commit_count == 1 { "" } else { "s" };
    let commits = commits
        .iter()
        .map(|c| {
            let th::PayloadCommit {
                id, message, url, ..
            } = c.deref();
            format!("[`{}`]({}) {}", &id[0..7], url, message)
        })
        .collect::<Vec<_>>()
        .join("\n");
    formatdoc! {
        r#"
            [{}:{}] {} commit{} was pushed by {}
            {}
        "#,
        repo_str(repo), r#ref, commit_count, commit_unit, user_str(sender), commits
    }
}

/// X-Gitea-Event: issues
fn issues(payload: th::IssuePayload) -> String {
    let th::IssuePayload {
        action,
        index,
        issue,
        repository: repo,
        sender,
        ..
    } = &payload;
    formatdoc! {
        r#"
            [{}] issue [#{} {}]({}) {} by {}
        "#,
        repo_str(repo), index, &issue.title, &issue.html_url, action, user_str(sender)
    }
}

fn pull_request(payload: th::PullRequestPayload) -> String {
    let th::PullRequestPayload {
        action,
        pull_request,
        repository: repo,
        sender,
        ..
    } = &payload;
    formatdoc! {
        r#"
            [{}] Pull Request {} {} by {}
        "#,
        repo_str(repo), pr_str(pull_request), action, user_str(sender)
    }
}

/// X-Gitea-Event: *
fn default(_event_type: &str, _payload: Value) -> Option<String> {
    None
}

fn repo_str(repo: &th::Repository) -> String {
    format!("[{}]({})", repo.full_name, repo.html_url)
}

fn user_str(user: &th::User) -> String {
    format!("[{}]({})", user.user_name, user.avatar_url)
}

fn pr_str(pr: &th::PullRequest) -> String {
    let th::PullRequest {
        id,
        title,
        html_url,
        ..
    } = pr;
    format!("[#{} {}]({})", id, title, html_url)
}
