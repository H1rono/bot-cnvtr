use axum::http::HeaderMap;
use indoc::formatdoc;
use serde_json::Value;

use super::utils::ValueExt;
use crate::{Error, Result};

pub(super) fn handle(headers: HeaderMap, payload: Value) -> Result<Option<String>> {
    let event_type = headers
        .get("X-GitHub-Event")
        .ok_or(Error::BadRequest)?
        .to_str()
        .map_err(|_| Error::BadRequest)?;
    match event_type {
        "create" => create(payload),
        "delete" => delete(payload),
        "push" => push(payload),
        "issues" => issues(payload),
        "ping" => ping(payload),
        "fork" => fork(payload),
        "branch_protection_rule" => branch_protection_rule(payload),
        _ => default(event_type, payload),
    }
}

/// X-GitHub-Event: ping
fn ping(_: Value) -> Result<Option<String>> {
    Ok(None)
}

/// X-GitHub-Event: create
fn create(payload: Value) -> Result<Option<String>> {
    let ref_name = payload.get_or_err("ref")?;
    let ref_type = payload.get_or_err("ref_type")?;
    let repo = payload.get_or_err("repository")?;
    let sender = payload.get_or_err("sender")?;
    let message = formatdoc! {
        r##"
            [{}] {} `{}` was created by {}.
        "##,
        repo_str(repo)?, ref_type, ref_name, user_str(sender)?
    };
    Ok(Some(message))
}

/// X-GitHub-Event: delete
fn delete(payload: Value) -> Result<Option<String>> {
    let ref_name = payload.get_or_err("ref")?;
    let ref_type = payload.get_or_err("ref_type")?;
    let repo = payload.get_or_err("repository")?;
    let sender = payload.get_or_err("sender")?;
    let message = formatdoc! {
        r##"
            [{}] {} `{}` was deleted by {}.
        "##,
        repo_str(repo)?, ref_type, ref_name, user_str(sender)?
    };
    Ok(Some(message))
}

/// X-GitHub-Event: push
fn push(payload: Value) -> Result<Option<String>> {
    let ref_name = payload.get_or_err("ref")?.as_str_or_err()?;
    let repo = payload.get_or_err("repository")?;
    let sender = payload.get_or_err("sender")?;
    let commits = payload.get_or_err("commits")?.as_array_or_err()?;
    let commit_count = commits.len();
    let commits = commits
        .iter()
        .map(|c| {
            let id = c.get_or_err("id")?.as_str_or_err()?;
            let message = c
                .get_or_err("message")?
                .as_str_or_err()?
                .lines()
                .next()
                .unwrap();
            let url = c.get_or_err("url")?.as_str_or_err()?;
            Ok(format!("[`{}`]({}) {}", &id[0..7], url, message))
        })
        .collect::<Result<Vec<_>>>()?
        .join("\n");
    let message = formatdoc! {
        r##"
            [{}:{}] {} commit(s) was pushed by {}
            {}
        "##,
        repo_str(repo)?, ref_name, commit_count, user_str(sender)?, commits
    };
    Ok(Some(message))
}

/// X-GitHub-Event: issues
fn issues(payload: Value) -> Result<Option<String>> {
    let action = payload.get_or_err("action")?.as_str_or_err()?;
    let repo = payload.get_or_err("repository")?;
    let sender = payload.get_or_err("sender")?;
    let issue = payload.get_or_err("issue")?;
    let issue_number = issue.get_or_err("number")?.as_u64_or_err()?;
    let issue_title = issue.get_or_err("title")?.as_str_or_err()?;
    let issue_url = issue.get_or_err("html_url")?.as_str_or_err()?;
    let message = formatdoc! {
        r##"
            [{}] Issue [`#{} {}`]({}) {} by {}
        "##,
        repo_str(repo)?,
        issue_number, issue_title, issue_url,
        action,
        user_str(sender)?
    };
    Ok(Some(message))
}

/// X-GitHub-Event: fork
fn fork(payload: Value) -> Result<Option<String>> {
    let forkee = payload.get_or_err("forkee")?;
    let repo = payload.get_or_err("repository")?;
    let sender = payload.get_or_err("sender")?;
    let message = formatdoc! {
        r##"
            [{}] forked to {} by {}
        "##,
        repo_str(repo)?,
        repo_str(forkee)?,
        user_str(sender)?
    };
    Ok(Some(message))
}

/// X-GitHub-Event: branch_protection_rule
fn branch_protection_rule(payload: Value) -> Result<Option<String>> {
    let action = payload.get_or_err("action")?.as_str_or_err()?;
    let branch = payload
        .get_or_err("rule")?
        .get_or_err("name")?
        .as_str_or_err()?;
    let repo = payload.get_or_err("repository")?;
    let sender = payload.get_or_err("sender")?;
    let message = formatdoc! {
        r##"
            [{}:{}] branch protection rule {} by {}
        "##,
        repo_str(repo)?,
        branch,
        action,
        user_str(sender)?
    };
    Ok(Some(message))
}

/// X-GitHub-Event: *
fn default(_event_type: &str, _payload: Value) -> Result<Option<String>> {
    Ok(None)
}

/// user -> [user.login](user.html_url)
fn user_str(user: &Value) -> Result<String> {
    let (name, url) = user_info(user)?;
    Ok(format!("[{}]({})", name, url))
}

/// user -> user.login, user.html_url
fn user_info(user: &Value) -> Result<(&str, &str)> {
    let name = user.get_or_err("login")?.as_str_or_err()?;
    let url = user.get_or_err("html_url")?.as_str_or_err()?;
    Ok((name, url))
}

/// repository -> [repository.full_name](repository.html_url)
fn repo_str(repo: &Value) -> Result<String> {
    let (name, url) = repo_info(repo)?;
    Ok(format!("[{}]({})", name, url))
}

/// repository -> repository.full_name, repository.html_url
fn repo_info(repo: &Value) -> Result<(&str, &str)> {
    let name = repo.get_or_err("full_name")?.as_str_or_err()?;
    let url = repo.get_or_err("html_url")?.as_str_or_err()?;
    Ok((name, url))
}
