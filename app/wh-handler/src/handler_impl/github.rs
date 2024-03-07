use std::str::from_utf8;

use github_webhook::payload_types as gh;
use http::HeaderMap;
use indoc::formatdoc;
use itertools::Itertools;
use paste::paste;
use serde_json::Value;

use domain::{Error, Event, EventSubscriber, Infra, Webhook};

use super::utils::extract_header_value;
use crate::WebhookHandlerImpl;

impl WebhookHandlerImpl {
    pub(crate) async fn handle_github<I>(
        &self,
        infra: &I,
        webhook: Webhook,
        headers: HeaderMap,
        payload: &str,
    ) -> Result<(), Error>
    where
        I: Infra,
        Error: From<I::Error>,
    {
        let subscriber = infra.event_subscriber();
        let Some(message) = handle(headers, payload)? else {
            return Ok(());
        };
        let kind = "github".to_string(); // TODO: event_type
        let event = Event {
            channel_id: webhook.channel_id,
            kind,
            body: message,
        };
        subscriber.send(event).await?;
        Ok(())
    }
}

#[tracing::instrument(target = "wh_handler::github::handle", skip_all)]
fn handle(headers: HeaderMap, payload: &str) -> Result<Option<String>, Error> {
    macro_rules! match_event {
        ($t:expr => $p:expr; $($i:ident),*) => {
            match $t {
                $(stringify!($i) => $i(from_str($p).map_err(anyhow::Error::from)?),)*
                _ => default($t, from_str($p).map_err(anyhow::Error::from)?),
            }
        };
    }

    use serde_json::from_str;
    let event_type = extract_header_value(&headers, "X-GitHub-Event")
        .and_then(|v| from_utf8(v).map_err(|_| Error::BadRequest))?;
    tracing::info!("X-GitHub-Event: {}", event_type);
    let message = match_event!(
        event_type => payload;
        create, delete, push, issues, issue_comment,
        ping, fork, release,
        branch_protection_rule,
        pull_request, pull_request_review_comment,
        pull_request_review, pull_request_review_thread,
        repository,
        star,
        workflow_run, workflow_job
    );
    Ok(message)
}

/// X-GitHub-Event: ping
fn ping(_: Value) -> Option<String> {
    None
}

/// X-GitHub-Event: create
fn create(payload: gh::CreateEvent) -> Option<String> {
    let gh::CreateEvent {
        ref_: ref_name,
        ref_type,
        repository,
        sender,
        ..
    } = &payload;
    let message = formatdoc! {
        r##"
            [{}] {} `{}` was created by {}
        "##,
        repo_str(repository), ser_ref_type(ref_type), ref_name, user_str(sender)
    };
    Some(message)
}

/// X-GitHub-Event: delete
fn delete(payload: gh::DeleteEvent) -> Option<String> {
    let ref_name = payload.ref_;
    let ref_type = payload.ref_type;
    let repo = payload.repository;
    let sender = payload.sender;
    let message = formatdoc! {
        r##"
            [{}] {} `{}` was deleted by {}
        "##,
        repo_str(&repo), ser_ref_type(&ref_type), ref_name, user_str(&sender)
    };
    Some(message)
}

/// X-GitHub-Event: push
fn push(payload: gh::PushEvent) -> Option<String> {
    let gh::PushEvent {
        ref_: ref_name,
        commits,
        repository: repo,
        sender,
        ..
    } = &payload;
    let commit_count = commits.len();
    let commit_unit = if commit_count == 1 { "" } else { "s" };
    let commits = commits
        .iter()
        .map(|c| {
            let gh::Commit {
                id, url, message, ..
            } = c;
            let message = message.lines().next().unwrap();
            format!("[`{}`]({}) {}", &id[0..7], url, message.trim_end())
        })
        .collect::<Vec<_>>()
        .join("\n");
    let message = formatdoc! {
        r##"
            [{}:{}] {} commit{} was pushed by {}
            {}
        "##,
        repo_str(repo), ref_name, commit_count, commit_unit,
        user_str(sender), commits
    };
    Some(message)
}

/// X-GitHub-Event: issues
fn issues(payload: gh::IssuesEvent) -> Option<String> {
    macro_rules! issue_event {
        ($i:ident, $kind:ident) => {{
            paste! {
                let gh::[< Issues $kind:camel Event >] {
                    repository, sender, issue, ..
                } = $i;
                (stringify!([< $kind:snake:lower >]), repository, sender, issue)
            }
        }};
    }

    macro_rules! issue_event_nested {
        ($i:ident, $kind:ident) => {{
            paste! {
                let gh::[< Issues $kind:camel Event >] {
                    repository, sender, issue, ..
                } = $i;
                (stringify!([< $kind:snake:lower >]), repository, sender, &issue.issue)
            }
        }};
    }

    use gh::IssuesEvent::*;

    let (action, repository, sender, issue) = match &payload {
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
    let message_headline = format!(
        "[{}] Issue {} {} by {}",
        repo_str(repository),
        issue_str(issue),
        action,
        user_str(sender)
    );
    let message_body = issue.body.as_deref().unwrap_or(&issue.html_url);
    let message_body_lines = message_body.lines().collect::<Vec<_>>();
    let message_body = if message_body_lines.len() > 5 {
        &issue.html_url
    } else {
        message_body
    };
    let message = format!("{}\n{}", message_headline, message_body);
    Some(message)
}

/// X-GitHub-Event: issue_comment
fn issue_comment(payload: gh::IssueCommentEvent) -> Option<String> {
    macro_rules! issue_comment {
        ($i:ident, $kind:ident) => {{
            paste! {
                let gh::[< IssueComment $kind:camel Event >] {
                    repository,
                    sender,
                    issue,
                    comment,
                    ..
                } = $i;
                let issue = &issue.issue;
                (stringify!($kind), repository, sender, issue, comment)
            }
        }};
    }

    use gh::IssueCommentEvent::*;

    let (action, repo, sender, issue, comment) = match &payload {
        Created(c) => issue_comment!(c, created),
        Edited(e) => issue_comment!(e, edited),
        Deleted(d) => issue_comment!(d, deleted),
    };
    let message_body_lines = comment.body.lines().collect::<Vec<_>>();
    let message_body = if message_body_lines.len() > 5 {
        "..."
    } else {
        &comment.body
    };
    Some(formatdoc! {
        r#"
            [{}] Issue {}: comment [{}]({}) by {}
            {}
        "#,
        repo_str(repo), issue_str(issue), action, comment.html_url, user_str(sender),
        message_body
    })
}

/// X-GitHub-Event: fork
fn fork(payload: gh::ForkEvent) -> Option<String> {
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
        repo_str(repository),
        repo_str(forkee),
        user_str(sender)
    };
    Some(message)
}

/// X-GitHub-Event: branch_protection_rule
fn branch_protection_rule(payload: gh::BranchProtectionRuleEvent) -> Option<String> {
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
        repo_str(repository),
        rule.name,
        action,
        user_str(sender)
    };
    Some(message)
}

/// X-GitHub-Event: pull_request
fn pull_request(payload: gh::PullRequestEvent) -> Option<String> {
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

    let message_headline = format!(
        "[{}] Pull Request {} {} by {}",
        repo_str(repository),
        pr_str(pull_request),
        action.replace('_', " "),
        user_str(sender)
    );
    let message_body = pull_request
        .body
        .as_deref()
        .unwrap_or(&pull_request.html_url);
    let message_body_lines = message_body.lines().collect::<Vec<_>>();
    let message_body = if message_body_lines.len() > 5 {
        &pull_request.html_url
    } else {
        message_body
    };

    let message = format!("{}\n{}", message_headline, message_body);
    Some(message)
}

/// X-GitHub-Event: pull_request_review_comment
fn pull_request_review_comment(payload: gh::PullRequestReviewCommentEvent) -> Option<String> {
    macro_rules! pr_review_comment_event {
        ($i:ident, $kind:ident) => {{
            paste! {
                let gh::[< PullRequestReviewComment $kind:camel Event >] {
                    repository, sender, pull_request, comment, ..
                } = $i;
                let pull_request = (pull_request.number, &pull_request.title, &pull_request.html_url);
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
            [{}] Pull Request comment {} in [#{} {}]({}) by {}
            {}
        "##,
        repo_str(repository),
        action,
        number, title, url,
        user_str(sender),
        comment.html_url
    };
    Some(message)
}

/// X-GitHub-Event: pull_request_review
fn pull_request_review(payload: gh::PullRequestReviewEvent) -> Option<String> {
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
        repo_str(repository),
        simple_pr_str(pull_request),
        action.replace('_', " "),
        user_str(sender),
        review.html_url
    };
    Some(message)
}

/// X-GitHub-Event: pull_request_review_thread
fn pull_request_review_thread(payload: gh::PullRequestReviewThreadEvent) -> Option<String> {
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
        repo_str(repository),
        simple_pr_str(pull_request),
        action.replace('_', " "),
        user_str(sender)
    };
    Some(message)
}

/// X-GitHub-Event: release
fn release(payload: gh::ReleaseEvent) -> Option<String> {
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
        repo_str(repository),
        release_str(release),
        action,
        user_str(sender)
    };
    Some(message)
}

/// X-GitHub-Event: repository
fn repository(payload: gh::RepositoryEvent) -> Option<String> {
    macro_rules! repository_event {
        ($i:ident, $kind:ident) => {{
            paste! {
                let gh::[< Repository $kind:camel Event >] {
                    repository, sender, ..
                } = $i;
                (repository, sender, stringify!([< $kind:snake:lower >]))
            }
        }};
    }

    use gh::RepositoryEvent::*;

    let (repository, sender, action) = match &payload {
        Created(r) => repository_event!(r, created),
        Archived(r) => repository_event!(r, archived),
        Deleted(r) => repository_event!(r, deleted),
        Edited(r) => repository_event!(r, edited),
        Privatized(r) => repository_event!(r, privatized),
        Publicized(r) => repository_event!(r, publicized),
        Renamed(r) => repository_event!(r, renamed),
        Transferred(r) => repository_event!(r, transferred),
        Unarchived(r) => repository_event!(r, unarchived),
    };
    Some(format!(
        "Repository {} {} by {}",
        repo_str(repository),
        action,
        user_str(sender)
    ))
}

/// X-GitHub-Event: star
fn star(payload: gh::StarEvent) -> Option<String> {
    let gh::StarEvent::Created(star) = &payload else {
        return None; // FIXME: deleteを伝えるなんてできない...
    };
    let gh::StarCreatedEvent {
        repository, sender, ..
    } = star;
    let message = formatdoc! {
        r#"
            [{}] :star: Repository starred by {} :star:
        "#,
        repo_str(repository), user_str(sender)
    };
    Some(message)
}

/// X-GitHub-Event: workflow_job
fn workflow_job(payload: gh::WorkflowJobEvent) -> Option<String> {
    use gh::WorkflowJobEvent::*;
    let message = match &payload {
        Completed(p) => {
            use gh::WorkflowJobCompletedEventWorkflowJobConclusion as Conclusion;
            let gh::WorkflowJobCompletedEvent {
                repository,
                workflow_job,
                ..
            } = p;
            let conclusion = match workflow_job.conclusion {
                Conclusion::Cancelled => "cancelled",
                Conclusion::Failure => "failed",
                Conclusion::Skipped => "skipped",
                Conclusion::Success => "success",
            };
            let workflow_job = &workflow_job.workflow_job;
            let steps = workflow_steps_str(&workflow_job.steps);
            formatdoc! {
                r#"
                    [{}] workflow job {} completed as {}
                    {}
                "#,
                repo_str(repository), workflow_job_str(workflow_job), conclusion, steps
            }
        }
        InProgress(p) => {
            use gh::WorkflowJobInProgressEventWorkflowJobStatus as Status;
            let gh::WorkflowJobInProgressEvent {
                repository,
                workflow_job,
                ..
            } = p;
            let status = match workflow_job.status {
                Status::InProgress => "in progress",
                Status::Queued => "queued",
            };
            let workflow_job = &workflow_job.workflow_job;
            let steps = &workflow_job.steps;
            if steps.is_empty() {
                formatdoc! {
                    r#"
                        [{}] workflow job {} {}
                    "#,
                    repo_str(repository), workflow_job_str(workflow_job), status
                }
            } else {
                let steps = workflow_steps_str(steps);
                formatdoc! {
                    r#"
                    [{}] workflow job {} {}
                    {}
                "#,
                    repo_str(repository), workflow_job_str(workflow_job), status, steps
                }
            }
        }
        Queued(p) => {
            use gh::WorkflowJobQueuedEventWorkflowJobStatus as Status;
            let gh::WorkflowJobQueuedEvent {
                repository,
                workflow_job,
                ..
            } = p;
            let status = match workflow_job.status {
                Status::Queued => "queued",
                Status::Waiting => "waiting",
            };
            let workflow_job = &workflow_job.workflow_job;
            formatdoc! {
                r#"
                    [{}] workflow job {} {}
                "#,
                repo_str(repository), workflow_job_str(workflow_job), status
            }
        }
        Waiting(p) => {
            let gh::WorkflowJobWaitingEvent {
                repository,
                workflow_job,
                ..
            } = p;
            formatdoc! {
                r#"
                    [{}] workflow job {} waiting
                "#,
                repo_str(repository), workflow_job_str(workflow_job)
            }
        }
    };
    Some(message)
}

/// X-GitHub-Event: workflow_run
fn workflow_run(payload: gh::WorkflowRunEvent) -> Option<String> {
    use gh::WorkflowRunEvent::{Completed, InProgress, Requested};
    let message = match payload {
        Completed(p) => {
            use gh::CheckRunCompletedEventCheckRunConclusion as Conclusion;
            let gh::WorkflowRunCompletedEvent {
                repository,
                workflow,
                workflow_run,
                ..
            } = &p;
            let workflow_run = &workflow_run.workflow_run;
            let Some(conclusion) = &workflow_run.conclusion else {
                return None;
            };
            let conclusion = match conclusion {
                Conclusion::ActionRequired => "action required",
                Conclusion::Cancelled => "cancelled",
                Conclusion::Failure => "failed",
                Conclusion::Neutral => "neutral",
                Conclusion::Skipped => "skipped",
                Conclusion::Stale => "stale",
                Conclusion::Success => "success",
                Conclusion::TimedOut => "timed out",
            };
            formatdoc! {
                r#"
                    [{}:{}] Workflow run {} / {} completed as {}
                "#,
                repo_str(repository), workflow_run.head_branch,
                workflow_str(workflow), workflow_run_str(workflow_run), conclusion
            }
        }
        InProgress(p) => {
            let gh::WorkflowRunInProgressEvent {
                repository,
                workflow,
                workflow_run,
                ..
            } = &p;
            formatdoc! {
                r#"
                    [{}:{}] Workflow run {} / {} is running
                "#,
                repo_str(repository), workflow_run.head_branch,
                workflow_str(workflow), workflow_run_str(workflow_run)
            }
        }
        Requested(p) => {
            let gh::WorkflowRunRequestedEvent {
                repository,
                sender,
                workflow,
                workflow_run,
                ..
            } = &p;
            formatdoc! {
                r#"
                    [{}:{}] Workflow run {} / {} requested by {}
                "#,
                repo_str(repository), workflow_run.head_branch,
                workflow_str(workflow), workflow_run_str(workflow_run),
                user_str(sender)
            }
        }
    };
    Some(message)
}

/// X-GitHub-Event: *
fn default(_event_type: &str, _payload: Value) -> Option<String> {
    None
}

/// user -> `[user.login](user.html_url)`
fn user_str(user: &gh::User) -> String {
    let gh::User {
        login, html_url, ..
    } = user;
    format!("[{}]({})", login, html_url)
}

/// repository -> `[repository.full_name](repository.html_url)`
fn repo_str(repo: &gh::Repository) -> String {
    let gh::Repository {
        full_name,
        html_url,
        ..
    } = repo;
    format!("[{}]({})", full_name, html_url)
}

fn ser_ref_type(rt: &gh::CreateEventRefType) -> &str {
    match rt {
        gh::CreateEventRefType::Branch => "branch",
        gh::CreateEventRefType::Tag => "tag",
    }
}

/// pr -> `[#pr.number pr.title](pr.html_url)`
fn pr_str(pr: &gh::PullRequest) -> String {
    let gh::PullRequest {
        number,
        title,
        html_url,
        ..
    } = pr;
    format!("[#{} {}]({})", number, title, html_url)
}

/// `[#issue.number issue.title](issue.html_url)`
fn issue_str(issue: &gh::Issue) -> String {
    let gh::Issue {
        number,
        title,
        html_url,
        ..
    } = issue;
    format!("[#{} {}]({})", number, title, html_url)
}

/// pr -> `[#pr.number pr.title](pr.html_url)`
fn simple_pr_str(pr: &gh::SimplePullRequest) -> String {
    let gh::SimplePullRequest {
        number,
        title,
        html_url,
        ..
    } = pr;
    format!("[#{} {}]({})", number, title, html_url)
}

/// release -> `[release.name](release.html_url)`
fn release_str(release: &gh::Release) -> String {
    let gh::Release { name, html_url, .. } = release;
    format!("[{}]({})", name, html_url)
}

/// workflow -> `[workflow.name](workflow.html_url)`
fn workflow_str(workflow: &gh::Workflow) -> String {
    let gh::Workflow { name, html_url, .. } = workflow;
    format!("[{}]({})", name, html_url)
}

/// workflow_run -> `[workflow_run.diplay_title](worlflow_run.html_url)`
fn workflow_run_str(workflow_run: &gh::WorkflowRun) -> String {
    let gh::WorkflowRun {
        display_title,
        html_url,
        ..
    } = workflow_run;
    format!("[{}]({})", display_title, html_url)
}

/// workflow_job -> `[workflow_job.workflow_name / workflow_job.name](workflow_job.html_url)`
fn workflow_job_str(workflow_job: &gh::WorkflowJob) -> String {
    let gh::WorkflowJob {
        workflow_name,
        name,
        html_url,
        ..
    } = workflow_job;
    match workflow_name.as_deref() {
        None => format!("[{}]({})", name, html_url),
        Some(workflow_name) => format!("[{} / {}]({})", workflow_name, name, html_url),
    }
}

fn workflow_steps_str<'a, I>(steps: I) -> String
where
    I: IntoIterator<Item = &'a gh::WorkflowStep>,
{
    use gh::WorkflowStep::*;
    steps
        .into_iter()
        .map(|step| match step {
            Completed(s) => {
                use gh::WorkflowStepCompletedConclusion as Conclusion;
                let gh::WorkflowStepCompleted {
                    number,
                    name,
                    conclusion,
                    ..
                } = s;
                let conclusion = match conclusion {
                    Conclusion::Failure => "failed",
                    Conclusion::Skipped => "skipped",
                    Conclusion::Success => "success",
                };
                format!("{}. `{}`: completed as {}", number, name, conclusion)
            }
            InProgress(s) => {
                let gh::WorkflowStepInProgress { number, name, .. } = s;
                format!("{}. `{}`: in progress", number, name)
            }
            Queued(s) => {
                let gh::WorkflowStepQueued { number, name, .. } = s;
                format!("{}. `{}`: queued", number, name)
            }
        })
        .join("\n")
}
