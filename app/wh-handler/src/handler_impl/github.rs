use std::str::from_utf8;

use http::HeaderMap;
use indoc::formatdoc;
use serde_json::Value;

use super::utils::{extract_header_value, ValueExt};
use crate::{Error, Result};

pub(super) fn handle(headers: HeaderMap, payload: Value) -> Result<Option<String>> {
    let event_type = extract_header_value(&headers, "X-GitHub-Event")
        .and_then(|v| from_utf8(v).map_err(|_| Error::WrongType))?;
    match event_type {
        "create" => create(payload),
        "delete" => delete(payload),
        "push" => push(payload),
        "issues" => issues(payload),
        "ping" => ping(payload),
        "fork" => fork(payload),
        "branch_protection_rule" => branch_protection_rule(payload),
        "pull_request" => pull_request(payload),
        "pull_request_comment" => pull_request_comment(payload),
        "pull_request_review" => pull_request_review(payload),
        "pull_request_review_thread" => pull_request_review_thread(payload),
        "release" => release(payload),
        _ => default(event_type, payload),
    }
}

/// X-GitHub-Event: ping
fn ping(_: Value) -> Result<Option<String>> {
    Ok(None)
}

/// X-GitHub-Event: create
fn create(payload: Value) -> Result<Option<String>> {
    let ref_name = payload.get_or_err("ref")?.as_str_or_err()?;
    let ref_type = payload.get_or_err("ref_type")?.as_str_or_err()?;
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
    let ref_name = payload.get_or_err("ref")?.as_str_or_err()?;
    let ref_type = payload.get_or_err("ref_type")?.as_str_or_err()?;
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

/// X-GitHub-Event: pull_request
fn pull_request(payload: Value) -> Result<Option<String>> {
    let action = payload.get_or_err("action")?.as_str_or_err()?;
    let repo = payload.get_or_err("repository")?;
    let sender = payload.get_or_err("sender")?;
    let pull_request = payload.get_or_err("pull_request")?;
    let message = formatdoc! {
        r##"
            [{}] Pull Request {} {} by {}
        "##,
        repo_str(repo)?,
        pr_str(pull_request)?,
        action.replace('_', " "),
        user_str(sender)?
    };
    Ok(Some(message))
}

/// X-GitHub-Event: pull_request_comment
fn pull_request_comment(payload: Value) -> Result<Option<String>> {
    let action = payload.get_or_err("action")?.as_str_or_err()?;
    let repo = payload.get_or_err("repository")?;
    let sender = payload.get_or_err("sender")?;
    let pull_request = payload.get_or_err("pull_request")?;
    let comment_url = payload
        .get_or_err("comment")?
        .get_or_err("html_url")?
        .as_str_or_err()?;
    let message = formatdoc! {
        r##"
            [{}] Pull Request comment {} in {} by {}
            {}
        "##,
        repo_str(repo)?,
        action,
        pr_str(pull_request)?,
        user_str(sender)?,
        comment_url
    };
    Ok(Some(message))
}

/// X-GitHub-Event: pull_request_review
fn pull_request_review(payload: Value) -> Result<Option<String>> {
    let action = payload.get_or_err("action")?.as_str_or_err()?;
    let repo = payload.get_or_err("repository")?;
    let sender = payload.get_or_err("sender")?;
    let pull_request = payload.get_or_err("pull_request")?;
    let review_url = payload
        .get_or_err("review")?
        .get_or_err("html_url")?
        .as_str_or_err()?;
    let message = formatdoc! {
        r##"
            [{}] Pull Request review {} {} by {}
            {}
        "##,
        repo_str(repo)?,
        pr_str(pull_request)?,
        action.replace('_', " "),
        user_str(sender)?,
        review_url
    };
    Ok(Some(message))
}

/// X-GitHub-Event: pull_request_review_thread
fn pull_request_review_thread(payload: Value) -> Result<Option<String>> {
    let action = payload.get_or_err("action")?.as_str_or_err()?;
    let repo = payload.get_or_err("repository")?;
    let sender = payload.get_or_err("sender")?;
    let pull_request = payload.get_or_err("pull_request")?;
    let message = formatdoc! {
        r##"
            [{}] Pull Request review thread {} {} by {}
        "##,
        repo_str(repo)?,
        pr_str(pull_request)?,
        action.replace('_', " "),
        user_str(sender)?
    };
    Ok(Some(message))
}

/// X-GitHub-Event: release
fn release(payload: Value) -> Result<Option<String>> {
    let action = payload.get_or_err("action")?.as_str_or_err()?;
    let repo = payload.get_or_err("repository")?;
    let sender = payload.get_or_err("sender")?;
    let release = payload.get_or_err("release")?;
    let message = formatdoc! {
        r##"
            [{}] Release {} {} by {}
        "##,
        repo_str(repo)?,
        release_str(release)?,
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

/// pr -> [`pr.number pr.title`](pr.html_url)
fn pr_str(pr: &Value) -> Result<String> {
    let (number, title, url) = pr_info(pr)?;
    Ok(format!("[`#{} {}`]({})", number, title, url))
}

/// pr -> pr.number, pr.title, pr.html_url
fn pr_info(pr: &Value) -> Result<(u64, &str, &str)> {
    let number = pr.get_or_err("number")?.as_u64_or_err()?;
    let title = pr.get_or_err("title")?.as_str_or_err()?;
    let url = pr.get_or_err("html_url")?.as_str_or_err()?;
    Ok((number, title, url))
}

/// release -> [release.name](release.html_url)
fn release_str(release: &Value) -> Result<String> {
    let (name, url) = release_info(release)?;
    Ok(format!("[{}]({})", name, url))
}

/// release -> release.name, release.html_url
fn release_info(release: &Value) -> Result<(&str, &str)> {
    let name = release.get_or_err("name")?.as_str_or_err()?;
    let url = release.get_or_err("html_url")?.as_str_or_err()?;
    Ok((name, url))
}
