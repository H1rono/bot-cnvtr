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
    let message = match event_type {
        "create" => create(payload),
        "delete" => delete(payload),
        "push" => push(payload),
        "issues" => issues(payload),
        "ping" => ping(payload),
        _ => default(event_type, payload),
    }?;
    Ok(message)
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
    let (repo_name, repo_url) = repo_info(repo)?;
    let sender = payload.get_or_err("sender")?;
    let (sender_name, sender_url) = user_info(sender)?;
    let message = formatdoc! {
        r##"
            [[{}]({})] {} `{}` was created by [{}]({}).
        "##,
        repo_name, repo_url, ref_type, ref_name, sender_name, sender_url
    };
    Ok(Some(message))
}

/// X-GitHub-Event: delete
fn delete(payload: Value) -> Result<Option<String>> {
    let ref_name = payload.get_or_err("ref")?;
    let ref_type = payload.get_or_err("ref_type")?;
    let repo = payload.get_or_err("repository")?;
    let (repo_name, repo_url) = repo_info(repo)?;
    let sender = payload.get_or_err("sender")?;
    let (sender_name, sender_url) = user_info(sender)?;
    let message = formatdoc! {
        r##"
            [[{}]({})] {} `{}` was deleted by [{}]({}).
        "##,
        repo_name, repo_url, ref_type, ref_name, sender_name, sender_url
    };
    Ok(Some(message))
}

/// X-GitHub-Event: push
fn push(payload: Value) -> Result<Option<String>> {
    let ref_name = payload.get_or_err("ref")?.as_str_or_err()?;
    let repo = payload.get_or_err("repository")?;
    let (repo_name, repo_url) = repo_info(repo)?;
    let sender = payload.get_or_err("sender")?;
    let (sender_name, sender_url) = user_info(sender)?;
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
            [[{}]({}):{}] {} commit(s) was pushed by [{}]({})
            {}
        "##,
        repo_name, repo_url, ref_name, commit_count, sender_name, sender_url, commits
    };
    Ok(Some(message))
}

/// X-GitHub-Event: issues
fn issues(payload: Value) -> Result<Option<String>> {
    let action = payload.get_or_err("action")?.as_str_or_err()?;
    let repo = payload.get_or_err("repository")?;
    let (repo_name, repo_url) = repo_info(repo)?;
    let sender = payload.get_or_err("sender")?;
    let (sender_name, sender_url) = user_info(sender)?;
    let issue = payload.get_or_err("issue")?;
    let issue_number = issue.get_or_err("number")?.as_u64_or_err()?;
    let issue_title = issue.get_or_err("title")?.as_str_or_err()?;
    let issue_url = issue.get_or_err("html_url")?.as_str_or_err()?;
    let message = formatdoc! {
        r##"
            [[{}]({})] Issue [`#{} {}`]({}) {} by [{}]({})
        "##,
        repo_name, repo_url,
        issue_number, issue_title, issue_url,
        action,
        sender_name, sender_url
    };
    Ok(Some(message))
}

fn default(event_type: &str, payload: Value) -> Result<Option<String>> {
    let action = payload.get("action").and_then(Value::as_str);
    let ev_action = if let Some(act) = action {
        format!("{} {}", event_type, act)
    } else {
        event_type.to_string()
    };
    let repo = payload.get_or_err("repository")?;
    let (repo_name, repo_url) = repo_info(repo)?;
    let message = formatdoc! {
        r##"
            [[{}]({})] {}
            詳細は現在工事中です :construction:
        "##,
        repo_name, repo_url,
        ev_action
    };
    Ok(Some(message))
}

/// user -> user.login, user.html_url
fn user_info(user: &Value) -> Result<(&str, &str)> {
    let name = user.get_or_err("login")?.as_str_or_err()?;
    let url = user.get_or_err("html_url")?.as_str_or_err()?;
    Ok((name, url))
}

/// repository -> repository.full_name, repository.html_url
fn repo_info(repo: &Value) -> Result<(&str, &str)> {
    let name = repo.get_or_err("full_name")?.as_str_or_err()?;
    let url = repo.get_or_err("html_url")?.as_str_or_err()?;
    Ok((name, url))
}
