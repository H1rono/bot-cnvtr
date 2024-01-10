use std::str::from_utf8;

use github_webhook::payload_types as gh;
use http::HeaderMap;
use indoc::formatdoc;
use paste::paste;
use serde_json::Value;

use super::utils::extract_header_value;
use crate::{Error, Result};

pub(super) fn handle(headers: HeaderMap, payload: &str) -> Result<Option<String>> {
    use serde_json::from_str;
    let event_type = extract_header_value(&headers, "X-GitHub-Event")
        .and_then(|v| from_utf8(v).map_err(|_| Error::WrongType))?;
    match event_type {
        "create" => create(from_str(payload)?),
        "delete" => delete(from_str(payload)?),
        "push" => push(from_str(payload)?),
        "issues" => issues(from_str(payload)?),
        "ping" => ping(from_str(payload)?),
        "fork" => fork(from_str(payload)?),
        "branch_protection_rule" => branch_protection_rule(from_str(payload)?),
        "pull_request" => pull_request(from_str(payload)?),
        "pull_request_review_comment" => pull_request_review_comment(from_str(payload)?),
        "pull_request_review" => pull_request_review(from_str(payload)?),
        "pull_request_review_thread" => pull_request_review_thread(from_str(payload)?),
        "release" => release(from_str(payload)?),
        _ => default(event_type, from_str(payload)?),
    }
}

/// X-GitHub-Event: ping
fn ping(_: Value) -> Result<Option<String>> {
    Ok(None)
}

/// X-GitHub-Event: create
fn create(payload: gh::CreateEvent) -> Result<Option<String>> {
    let gh::CreateEvent {
        ref_: ref_name,
        ref_type,
        repository,
        sender,
        ..
    } = &payload;
    let message = formatdoc! {
        r##"
            [{}] {} `{}` was created by {}.
        "##,
        repo_str(repository)?, ser_ref_type(ref_type), ref_name, user_str(sender)?
    };
    Ok(Some(message))
}

/// X-GitHub-Event: delete
fn delete(payload: gh::DeleteEvent) -> Result<Option<String>> {
    let ref_name = payload.ref_;
    let ref_type = payload.ref_type;
    let repo = payload.repository;
    let sender = payload.sender;
    let message = formatdoc! {
        r##"
            [{}] {} `{}` was deleted by {}.
        "##,
        repo_str(&repo)?, ser_ref_type(&ref_type), ref_name, user_str(&sender)?
    };
    Ok(Some(message))
}

/// X-GitHub-Event: push
fn push(payload: gh::PushEvent) -> Result<Option<String>> {
    let ref_name = payload.ref_;
    let repo = &payload.repository;
    let sender = payload.sender;
    let commits = payload.commits;
    let commit_count = commits.len();
    let commits = commits
        .iter()
        .map(|c| {
            let gh::Commit {
                id, url, message, ..
            } = c;
            Ok(format!("[`{}`]({}) {}", &id[0..7], url, message))
        })
        .collect::<Result<Vec<_>>>()?
        .join("\n");
    let message = formatdoc! {
        r##"
            [{}:{}] {} commit(s) was pushed by {}
            {}
        "##,
        repo_str(repo)?, ref_name, commit_count, user_str(&sender)?, commits
    };
    Ok(Some(message))
}

/// X-GitHub-Event: issues
fn issues(payload: gh::IssuesEvent) -> Result<Option<String>> {
    macro_rules! issue_event {
        ($i:ident, $kind:ident) => {{
            paste! {
                let gh::[< Issues $kind:camel Event >] {
                    repository, sender, issue, ..
                } = $i;
                let &gh::Issue {
                    number, title, url, ..
                } = issue;
                (stringify!([< $kind:snake:lower >]), repository, sender, number, title, url)
            }
        }};
    }

    macro_rules! issue_event_nested {
        ($i:ident, $kind:ident) => {{
            paste! {
                let gh::[< Issues $kind:camel Event >] {
                    repository, sender, issue, ..
                } = $i;
                let gh::Issue {
                    number, title, url, ..
                } = issue.issue;
                (stringify!([< $kind:snake:lower >]), repository, sender, number, title, url)
            }
        }};
    }

    use gh::IssuesEvent::*;

    let (action, repository, sender, issue_number, issue_title, issue_url) = match &payload {
        Assigned(i) => issue_event!(i, assigned),
        Closed(i) => issue_event_nested!(i, closed),
        Deleted(i) => issue_event!(i, deleted),
        Demilestoned(i) => issue_event_nested!(i, demilestoned),
        Edited(i) => issue_event!(i, edited),
        Labeled(i) => issue_event!(i, labeled),
        Locked(i) => issue_event_nested!(i, locked),
        Milestoned(i) => issue_event_nested!(i, milestoned),
        Opened(i) => issue_event_nested!(i, opened),
        Pinned(i) => issue_event!(i, pinned),
        Reopened(i) => issue_event!(i, reopened),
        Transferred(i) => issue_event!(i, transferred),
        Unassigned(i) => issue_event!(i, unassigned),
        Unlabeled(i) => issue_event!(i, unlabeled),
        Unlocked(i) => issue_event_nested!(i, unlocked),
        Unpinned(i) => issue_event!(i, unpinned),
    };
    let message = formatdoc! {
        r##"
            [{}] Issue [`#{} {}`]({}) {} by {}
        "##,
        repo_str(repository)?,
        issue_number, issue_title, issue_url,
        action,
        user_str(sender)?
    };
    Ok(Some(message))
}

/// X-GitHub-Event: fork
fn fork(payload: gh::ForkEvent) -> Result<Option<String>> {
    let gh::ForkEvent {
        forkee,
        repository,
        sender,
        ..
    } = &payload;
    let message = formatdoc! {
        r##"
            [{}] forked to {} by {}
        "##,
        repo_str(repository)?,
        repo_str(forkee)?,
        user_str(sender)?
    };
    Ok(Some(message))
}

/// X-GitHub-Event: branch_protection_rule
fn branch_protection_rule(payload: gh::BranchProtectionRuleEvent) -> Result<Option<String>> {
    macro_rules! branch_protection_rule_event {
        ($i:ident, $kind:ident) => {{
            paste! {
                let gh::[< BranchProtectionRule $kind:camel Event >] {
                    repository, sender, rule, ..
                } = $i;
                (stringify!([< $kind:snake:lower >]), repository, sender, rule)
            }
        }};
    }

    use gh::BranchProtectionRuleEvent::*;

    let (action, repository, sender, rule) = match &payload {
        Created(r) => branch_protection_rule_event!(r, created),
        Deleted(r) => branch_protection_rule_event!(r, deleted),
        Edited(r) => branch_protection_rule_event!(r, edited),
    };
    let message = formatdoc! {
        r##"
            [{}:{}] branch protection rule {} by {}
        "##,
        repo_str(repository)?,
        rule.name,
        action,
        user_str(sender)?
    };
    Ok(Some(message))
}

/// X-GitHub-Event: pull_request
fn pull_request(payload: gh::PullRequestEvent) -> Result<Option<String>> {
    macro_rules! pull_request_event {
        ($i:ident, $kind:ident) => {{
            paste! {
                let gh::[< PullRequest $kind:camel Event >] {
                    repository, sender, pull_request, ..
                } = $i;
                (stringify!([< $kind:snake:lower >]), repository, sender, pull_request)
            }
        }};
    }

    macro_rules! pull_request_event_nested {
        ($i:ident, $kind:ident) => {{
            paste! {
                let gh::[< PullRequest $kind:camel Event >] {
                    repository, sender, pull_request, ..
                } = $i;
                (stringify!([< $kind:snake:lower >]), repository, sender, &pull_request.pull_request)
            }
        }};
    }

    use gh::PullRequestEvent::*;

    let (action, repository, sender, pull_request) = match &payload {
        Assigned(pr) => pull_request_event!(pr, assigned),
        AutoMergeDisabled(pr) => pull_request_event!(pr, auto_merge_disabled),
        AutoMergeEnabled(pr) => pull_request_event!(pr, auto_merge_enabled),
        Closed(pr) => pull_request_event_nested!(pr, closed),
        ConvertedToDraft(pr) => pull_request_event_nested!(pr, converted_to_draft),
        Demilestoned(pr) => pull_request_event_nested!(pr, demilestoned),
        Dequeued(pr) => pull_request_event!(pr, dequeued),
        Edited(pr) => pull_request_event!(pr, edited),
        Enqueued(pr) => pull_request_event!(pr, enqueued),
        Labeled(pr) => pull_request_event!(pr, labeled),
        Locked(pr) => pull_request_event!(pr, locked),
        Milestoned(pr) => pull_request_event_nested!(pr, milestoned),
        Opened(pr) => pull_request_event_nested!(pr, opened),
        ReadyForReview(pr) => pull_request_event_nested!(pr, ready_for_review),
        Reopened(pr) => pull_request_event_nested!(pr, reopened),
        ReviewRequestRemoved(pr) => pull_request_event!(pr, review_request_removed),
        ReviewRequested(pr) => pull_request_event!(pr, review_requested),
        Synchronize(pr) => pull_request_event!(pr, synchronize),
        Unassigned(pr) => pull_request_event!(pr, unassigned),
        Unlabeled(pr) => pull_request_event!(pr, unlabeled),
        Unlocked(pr) => pull_request_event!(pr, unlocked),
    };

    let message = formatdoc! {
        r##"
            [{}] Pull Request {} {} by {}
        "##,
        repo_str(repository)?,
        pr_str(pull_request)?,
        action.replace('_', " "),
        user_str(sender)?
    };
    Ok(Some(message))
}

/// X-GitHub-Event: pull_request_review_comment
fn pull_request_review_comment(
    payload: gh::PullRequestReviewCommentEvent,
) -> Result<Option<String>> {
    macro_rules! pr_review_comment_event {
        ($i:ident, $kind:ident) => {{
            paste! {
                let gh::[< PullRequestReviewComment $kind:camel Event >] {
                    repository, sender, pull_request, comment, ..
                } = $i;
                let pull_request = (pull_request.number, pull_request.title, pull_request.html_url);
                (stringify!([< $kind:snake:lower >]), repository, sender, pull_request, comment)
            }
        }};
    }

    use gh::PullRequestReviewCommentEvent::*;

    let (action, repository, sender, pull_request, comment) = match &payload {
        Created(r) => pr_review_comment_event!(r, created),
        Deleted(r) => pr_review_comment_event!(r, deleted),
        Edited(r) => pr_review_comment_event!(r, edited),
    };
    let (number, title, url) = pull_request;
    let message = formatdoc! {
        r##"
            [{}] Pull Request comment {} in [`#{} {}`]({}) by {}
            {}
        "##,
        repo_str(repository)?,
        action,
        number, title, url,
        user_str(sender)?,
        comment.html_url
    };
    Ok(Some(message))
}

/// X-GitHub-Event: pull_request_review
fn pull_request_review(payload: gh::PullRequestReviewEvent) -> Result<Option<String>> {
    macro_rules! pr_review_event {
        ($i:ident, $kind:ident) => {{
            paste! {
                let gh::[< PullRequestReview $kind:camel Event >] {
                    repository, sender, pull_request, review, ..
                } = $i;
                (stringify!([< $kind:snake:lower >]), repository, sender, pull_request, review)
            }
        }};
    }

    use gh::PullRequestReviewEvent::*;

    let (action, repository, sender, pull_request, review) = match &payload {
        Dismissed(r) => pr_review_event!(r, dismissed),
        Edited(r) => pr_review_event!(r, edited),
        Submitted(r) => pr_review_event!(r, submitted),
    };
    let message = formatdoc! {
        r##"
            [{}] Pull Request review {} {} by {}
            {}
        "##,
        repo_str(repository)?,
        simple_pr_str(pull_request)?,
        action.replace('_', " "),
        user_str(sender)?,
        review.html_url
    };
    Ok(Some(message))
}

/// X-GitHub-Event: pull_request_review_thread
fn pull_request_review_thread(payload: gh::PullRequestReviewThreadEvent) -> Result<Option<String>> {
    macro_rules! pr_review_thread_event {
        ($i:ident, $kind:ident) => {{
            paste! {
                let gh::[< PullRequestReviewThread $kind:camel Event >] {
                    repository, sender, pull_request, ..
                } = $i;
                (stringify!([< $kind:snake:lower >]), repository, sender, pull_request)
            }
        }};
    }

    use gh::PullRequestReviewThreadEvent::*;

    let (action, repository, sender, pull_request) = match &payload {
        Resolved(rt) => pr_review_thread_event!(rt, resolved),
        Unresolved(rt) => pr_review_thread_event!(rt, unresolved),
    };
    let message = formatdoc! {
        r##"
            [{}] Pull Request review thread {} {} by {}
        "##,
        repo_str(repository)?,
        simple_pr_str(pull_request)?,
        action.replace('_', " "),
        user_str(sender)?
    };
    Ok(Some(message))
}

/// X-GitHub-Event: release
fn release(payload: gh::ReleaseEvent) -> Result<Option<String>> {
    macro_rules! release_event {
        ($i:ident, $kind:ident) => {{
            paste! {
                let gh::[< Release $kind:camel Event >] {
                    repository, sender, release, ..
                } = $i;
                (stringify!([< $kind:snake:lower >]), repository, sender, release)
            }
        }};
    }

    macro_rules! release_event_nested {
        ($i:ident, $kind:ident) => {{
            paste! {
                let gh::[< Release $kind:camel Event >] {
                    repository, sender, release, ..
                } = $i;
                (stringify!([< $kind:snake:lower >]), repository, sender, &release.release)
            }
        }};
    }

    use gh::ReleaseEvent::*;

    let (action, repository, sender, release) = match &payload {
        Created(r) => release_event!(r, created),
        Deleted(r) => release_event!(r, deleted),
        Edited(r) => release_event!(r, edited),
        Prereleased(r) => release_event!(r, prereleased),
        Published(r) => release_event_nested!(r, published),
        Released(r) => release_event!(r, released),
        Unpublished(r) => release_event_nested!(r, unpublished),
    };
    let message = formatdoc! {
        r##"
            [{}] Release {} {} by {}
        "##,
        repo_str(repository)?,
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
fn user_str(user: &gh::User) -> Result<String> {
    let &gh::User { name, html_url, .. } = user;
    Ok(format!("[{}]({})", name.unwrap_or("{anonymous}"), html_url))
}

/// repository -> [repository.full_name](repository.html_url)
fn repo_str(repo: &gh::Repository) -> Result<String> {
    let &gh::Repository { name, html_url, .. } = repo;
    Ok(format!("[{}]({})", name, html_url))
}

fn ser_ref_type(rt: &gh::CreateEventRefType) -> &str {
    match rt {
        gh::CreateEventRefType::Branch => "branch",
        gh::CreateEventRefType::Tag => "tag",
    }
}

/// pr -> [`pr.number pr.title`](pr.html_url)
fn pr_str(pr: &gh::PullRequest) -> Result<String> {
    let gh::PullRequest {
        number,
        title,
        html_url,
        ..
    } = pr;
    Ok(format!("[`#{} {}`]({})", number, title, html_url))
}

fn simple_pr_str(pr: &gh::SimplePullRequest) -> Result<String> {
    let gh::SimplePullRequest {
        number,
        title,
        html_url,
        ..
    } = pr;
    Ok(format!("[`#{} {}`]({})", number, title, html_url))
}

/// release -> [release.name](release.html_url)
fn release_str(release: &gh::Release) -> Result<String> {
    let &gh::Release { name, html_url, .. } = release;
    Ok(format!("[{}]({})", name, html_url))
}
