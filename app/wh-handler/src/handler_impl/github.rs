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
        let kind = "github".to_string().into(); // TODO: event_type
        let event = Event {
            channel_id: webhook.channel_id,
            kind,
            body: message.into(),
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
    tracing::info!("X-GitHub-Event: {event_type}");
    let message = match_event!(
        event_type => payload;
        create, delete, push, issues, issue_comment,
        ping, fork, release,
        branch_protection_rule,
        pull_request, pull_request_review_comment,
        pull_request_review, pull_request_review_thread,
        repository,
        star, watch,
        workflow_run, workflow_job
    );
    Ok(message)
}

/// `X-GitHub-Event: ping`
fn ping(_: Value) -> Option<String> {
    None
}

/// `X-GitHub-Event: create`
fn create(payload: gh::CreateEvent) -> Option<String> {
    let gh::CreateEvent {
        ref_: ref_name,
        ref_type,
        repository,
        sender,
        ..
    } = &payload;
    let repo = repo_str(repository);
    let ref_type = ser_ref_type(ref_type);
    let sender = user_str(sender);
    let message = formatdoc! {
        r##"
            [{repo}] {ref_type} `{ref_name}` was created by {sender}
        "##
    };
    Some(message)
}

/// `X-GitHub-Event: delete`
fn delete(payload: gh::DeleteEvent) -> Option<String> {
    let gh::DeleteEvent {
        ref_: ref_name,
        ref_type,
        repository,
        sender,
        ..
    } = &payload;
    let repo = repo_str(repository);
    let ref_type = ser_ref_type(ref_type);
    let sender = user_str(sender);
    let message = formatdoc! {
        r##"
            [{repo}] {ref_type} `{ref_name}` was deleted by {sender}
        "##
    };
    Some(message)
}

/// `X-GitHub-Event: push`
fn push(payload: gh::PushEvent) -> Option<String> {
    let gh::PushEvent {
        ref_: ref_name,
        commits,
        repository,
        sender,
        ..
    } = &payload;
    let repo = repo_str(repository);
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
    let sender = user_str(sender);
    let message = formatdoc! {
        r##"
            [{repo}:{ref_name}] {commit_count} commit{commit_unit} was pushed by {sender}
            {commits}
        "##
    };
    Some(message)
}

/// `X-GitHub-Event: issues`
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

    use gh::IssuesEvent as Ie;

    let (action, repository, sender, issue) = match &payload {
        Ie::Assigned(i) => issue_event!(i, assigned),
        Ie::Closed(i) => issue_event_nested!(i, closed),
        Ie::Deleted(i) => issue_event!(i, deleted),
        Ie::Demilestoned(i) => issue_event_nested!(i, demilestoned),
        Ie::Edited(i) => issue_event!(i, edited),
        Ie::Labeled(i) => issue_event!(i, labeled),
        Ie::Locked(i) => issue_event_nested!(i, locked),
        Ie::Milestoned(i) => issue_event_nested!(i, milestoned),
        Ie::Opened(i) => issue_event_nested!(i, opened),
        Ie::Pinned(i) => issue_event!(i, pinned),
        Ie::Reopened(i) => issue_event!(i, reopened),
        Ie::Transferred(i) => issue_event!(i, transferred),
        Ie::Unassigned(i) => issue_event!(i, unassigned),
        Ie::Unlabeled(i) => issue_event!(i, unlabeled),
        Ie::Unlocked(i) => issue_event_nested!(i, unlocked),
        Ie::Unpinned(i) => issue_event!(i, unpinned),
    };
    let message_headline = format!(
        "[{repo}] Issue {issue} {action} by {sender}",
        repo = repo_str(repository),
        issue = issue_str(issue),
        sender = user_str(sender)
    );
    let message_body = issue.body.as_deref().unwrap_or(&issue.html_url);
    let message_body_lines = message_body.lines().collect::<Vec<_>>();
    let message_body = if message_body_lines.len() > 5 {
        &issue.html_url
    } else {
        message_body
    };
    let message = format!("{message_headline}\n{message_body}");
    Some(message)
}

/// `X-GitHub-Event: issue_comment`
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

    use gh::IssueCommentEvent::{Created, Deleted, Edited};

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
    let message = formatdoc! {
        r#"
            [{repo}] Issue {issue}: comment [{action}]({comment_url}) by {sender}
            {message_body}
        "#,
        repo = repo_str(repo),
        issue = issue_str(issue),
        comment_url = comment.html_url,
        sender = user_str(sender)
    };
    Some(message)
}

/// `X-GitHub-Event: fork`
fn fork(payload: gh::ForkEvent) -> Option<String> {
    let gh::ForkEvent {
        forkee,
        repository,
        sender,
        ..
    } = &payload;
    let repo = repo_str(repository);
    let forkee = repo_str(forkee);
    let sender = user_str(sender);
    let message = format!("[{repo}] forked to {forkee} by {sender}\n");
    Some(message)
}

/// `X-GitHub-Event: branch_protection_rule`
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

    use gh::BranchProtectionRuleEvent::{Created, Deleted, Edited};

    let (action, repository, sender, rule) = match &payload {
        Created(r) => branch_protection_rule_event!(r, created),
        Deleted(r) => branch_protection_rule_event!(r, deleted),
        Edited(r) => branch_protection_rule_event!(r, edited),
    };
    let repo = repo_str(repository);
    let rule_name = &rule.name;
    let sender = user_str(sender);
    let message = formatdoc! {
        r##"
            [{repo}:{rule_name}] branch protection rule {action} by {sender}
        "##
    };
    Some(message)
}

/// `X-GitHub-Event: pull_request`
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

    use gh::PullRequestEvent as PRe;

    let (action, repository, sender, pull_request) = match &payload {
        PRe::Assigned(pr) => pull_request_event!(pr, assigned),
        PRe::AutoMergeDisabled(pr) => pull_request_event!(pr, auto_merge_disabled),
        PRe::AutoMergeEnabled(pr) => pull_request_event!(pr, auto_merge_enabled),
        PRe::Closed(pr) => pull_request_event_nested!(pr, closed),
        PRe::ConvertedToDraft(pr) => pull_request_event_nested!(pr, converted_to_draft),
        PRe::Demilestoned(pr) => pull_request_event_nested!(pr, demilestoned),
        PRe::Dequeued(pr) => pull_request_event!(pr, dequeued),
        PRe::Edited(pr) => pull_request_event!(pr, edited),
        PRe::Enqueued(pr) => pull_request_event!(pr, enqueued),
        PRe::Labeled(pr) => pull_request_event!(pr, labeled),
        PRe::Locked(pr) => pull_request_event!(pr, locked),
        PRe::Milestoned(pr) => pull_request_event_nested!(pr, milestoned),
        PRe::Opened(pr) => pull_request_event_nested!(pr, opened),
        PRe::ReadyForReview(pr) => pull_request_event_nested!(pr, ready_for_review),
        PRe::Reopened(pr) => pull_request_event_nested!(pr, reopened),
        PRe::ReviewRequestRemoved(pr) => pull_request_event!(pr, review_request_removed),
        PRe::ReviewRequested(pr) => pull_request_event!(pr, review_requested),
        PRe::Synchronize(pr) => pull_request_event!(pr, synchronize),
        PRe::Unassigned(pr) => pull_request_event!(pr, unassigned),
        PRe::Unlabeled(pr) => pull_request_event!(pr, unlabeled),
        PRe::Unlocked(pr) => pull_request_event!(pr, unlocked),
    };

    let message_headline = format!(
        "[{repo}] Pull Request {pr} {action} by {sender}",
        repo = repo_str(repository),
        pr = pr_str(pull_request),
        action = action.replace('_', " "),
        sender = user_str(sender)
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

    let message = format!("{message_headline}\n{message_body}");
    Some(message)
}

/// `X-GitHub-Event: pull_request_review_comment`
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

    use gh::PullRequestReviewCommentEvent::{Created, Deleted, Edited};

    let (action, repository, sender, pull_request, comment) = match &payload {
        Created(r) => pr_review_comment_event!(r, created),
        Deleted(r) => pr_review_comment_event!(r, deleted),
        Edited(r) => pr_review_comment_event!(r, edited),
    };
    let (number, title, url) = pull_request;
    let repo = repo_str(repository);
    let sender = user_str(sender);
    let pr = format!("[#{number} {title}]({url})");
    let comment_url = &comment.html_url;
    let message = formatdoc! {
        r##"
            [{repo}] Pull Request comment {action} in {pr} by {sender}
            {comment_url}
        "##,
    };
    Some(message)
}

/// `X-GitHub-Event: pull_request_review`
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

    use gh::PullRequestReviewEvent::{Dismissed, Edited, Submitted};

    let (action, repository, sender, pull_request, review) = match &payload {
        Dismissed(r) => pr_review_event!(r, dismissed),
        Edited(r) => pr_review_event!(r, edited),
        Submitted(r) => pr_review_event!(r, submitted),
    };
    let repo = repo_str(repository);
    let pr = simple_pr_str(pull_request);
    let action = action.replace('_', " ");
    let sender = user_str(sender);
    let review_url = &review.html_url;
    let message = formatdoc! {
        r##"
            [{repo}] Pull Request review {pr} {action} by {sender}
            {review_url}
        "##
    };
    Some(message)
}

/// `X-GitHub-Event: pull_request_review_thread`
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

    use gh::PullRequestReviewThreadEvent::{Resolved, Unresolved};

    let (action, repository, sender, pull_request) = match &payload {
        Resolved(rt) => pr_review_thread_event!(rt, resolved),
        Unresolved(rt) => pr_review_thread_event!(rt, unresolved),
    };
    let repo = repo_str(repository);
    let pr = simple_pr_str(pull_request);
    let action = action.replace('_', " ");
    let sender = user_str(sender);
    let message = format!("[{repo}] Pull Request review thread {pr} {action} by {sender}\n");
    Some(message)
}

/// `X-GitHub-Event: release`
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

    use gh::ReleaseEvent as Re;

    let (action, repository, sender, release) = match &payload {
        Re::Created(r) => release_event!(r, created),
        Re::Deleted(r) => release_event!(r, deleted),
        Re::Edited(r) => release_event!(r, edited),
        Re::Prereleased(r) => release_event!(r, prereleased),
        Re::Published(r) => release_event_nested!(r, published),
        Re::Released(r) => release_event!(r, released),
        Re::Unpublished(r) => release_event_nested!(r, unpublished),
    };
    let repo = repo_str(repository);
    let release = release_str(release);
    let sender = user_str(sender);
    let message = format!("[{repo}] Release {release} {action} by {sender}\n");
    Some(message)
}

/// `X-GitHub-Event: repository`
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

    use gh::RepositoryEvent as Re;

    let (repository, sender, action) = match &payload {
        Re::Created(r) => repository_event!(r, created),
        Re::Archived(r) => repository_event!(r, archived),
        Re::Deleted(r) => repository_event!(r, deleted),
        Re::Edited(r) => repository_event!(r, edited),
        Re::Privatized(r) => repository_event!(r, privatized),
        Re::Publicized(r) => repository_event!(r, publicized),
        Re::Renamed(r) => repository_event!(r, renamed),
        Re::Transferred(r) => repository_event!(r, transferred),
        Re::Unarchived(r) => repository_event!(r, unarchived),
    };
    let repo = repo_str(repository);
    let sender = user_str(sender);
    let message = format!("Repository {repo} {action} by {sender}\n");
    Some(message)
}

/// `X-GitHub-Event: star`
fn star(payload: gh::StarEvent) -> Option<String> {
    let gh::StarEvent::Created(star) = &payload else {
        return None; // FIXME: deleteを伝えるなんてできない...
    };
    let gh::StarCreatedEvent {
        repository, sender, ..
    } = star;
    let repo = repo_str(repository);
    let sender = user_str(sender);
    let message = format!("[{repo}] :star: Repository starred by {sender} :star:\n");
    Some(message)
}

/// `X-GitHub-Event: watch`
fn watch(payload: gh::WatchEvent) -> Option<String> {
    let gh::WatchEvent {
        repository, sender, ..
    } = &payload;
    let repo = repo_str(repository);
    let sender = user_str(sender);
    let message = format!("[{repo}] {sender} started watching\n");
    Some(message)
}

/// `X-GitHub-Event: workflow_job`
fn workflow_job(payload: gh::WorkflowJobEvent) -> Option<String> {
    use gh::WorkflowJobEvent::{Completed, InProgress, Queued, Waiting};
    let message = match &payload {
        Completed(p) => {
            use gh::WorkflowJobCompletedEventWorkflowJobConclusion as Conclusion;
            let gh::WorkflowJobCompletedEvent {
                repository,
                workflow_job,
                ..
            } = p;
            let repo = repo_str(repository);
            let job = workflow_job_str(&workflow_job.workflow_job);
            let conclusion = match workflow_job.conclusion {
                Conclusion::Cancelled => "cancelled",
                Conclusion::Failure => "failed",
                Conclusion::Skipped => "skipped",
                Conclusion::Success => "success",
            };
            let steps = workflow_steps_str(&workflow_job.workflow_job.steps);
            formatdoc! {
                r#"
                    [{repo}] workflow job {job} completed as {conclusion}
                    {steps}
                "#
            }
        }
        InProgress(p) => {
            use gh::WorkflowJobInProgressEventWorkflowJobStatus as Status;
            let gh::WorkflowJobInProgressEvent {
                repository,
                workflow_job,
                ..
            } = p;
            let repo = repo_str(repository);
            let job = workflow_job_str(&workflow_job.workflow_job);
            let status = match workflow_job.status {
                Status::InProgress => "in progress",
                Status::Queued => "queued",
            };
            let steps = &workflow_job.workflow_job.steps;
            if steps.is_empty() {
                formatdoc! {
                    r#"
                        [{repo}] workflow job {job} {status}
                    "#
                }
            } else {
                // FIXME
                let steps = workflow_steps_str(steps);
                formatdoc! {
                    r#"
                        [{repo}] workflow job {job} {status}
                        {steps}
                    "#
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
            let repo = repo_str(repository);
            let job = workflow_job_str(&workflow_job.workflow_job);
            let status = match workflow_job.status {
                Status::Queued => "queued",
                Status::Waiting => "waiting",
            };
            format!("[{repo}] workflow job {job} {status}\n")
        }
        Waiting(p) => {
            let gh::WorkflowJobWaitingEvent {
                repository,
                workflow_job,
                ..
            } = p;
            let repo = repo_str(repository);
            let job = workflow_job_str(workflow_job);
            format!("[{repo}] workflow job {job} waiting\n")
        }
    };
    Some(message)
}

/// `X-GitHub-Event: workflow_run`
fn workflow_run(payload: gh::WorkflowRunEvent) -> Option<String> {
    use gh::WorkflowRunEvent::{Completed, InProgress, Requested};
    let message = match payload {
        Completed(p) => {
            use gh::CheckRunCompletedEventCheckRunConclusion as Conclusion;
            let gh::WorkflowRunCompletedEvent {
                repository,
                workflow,
                workflow_run: gh::WorkflowRunCompletedEventWorkflowRun { workflow_run, .. },
                ..
            } = &p;
            let repo = repo_str(repository);
            let branch = &workflow_run.head_branch;
            let wf = workflow_str(workflow);
            let wf_run = workflow_run_str(workflow_run);
            let conclusion = match workflow_run.conclusion.as_ref()? {
                Conclusion::ActionRequired => "action required",
                Conclusion::Cancelled => "cancelled",
                Conclusion::Failure => "failed",
                Conclusion::Neutral => "neutral",
                Conclusion::Skipped => "skipped",
                Conclusion::Stale => "stale",
                Conclusion::Success => "success",
                Conclusion::TimedOut => "timed out",
            };
            format!("[{repo}:{branch}] Workflow run {wf} / {wf_run} completed as {conclusion}\n")
        }
        InProgress(p) => {
            let gh::WorkflowRunInProgressEvent {
                repository,
                workflow,
                workflow_run,
                ..
            } = &p;
            let repo = repo_str(repository);
            let branch = &workflow_run.head_branch;
            let wf = workflow_str(workflow);
            let wf_run = workflow_run_str(workflow_run);
            format!("[{repo}:{branch}] Workflow run {wf} / {wf_run} is running\n")
        }
        Requested(p) => {
            let gh::WorkflowRunRequestedEvent {
                repository,
                sender,
                workflow,
                workflow_run,
                ..
            } = &p;
            let repo = repo_str(repository);
            let branch = &workflow_run.head_branch;
            let wf = workflow_str(workflow);
            let wf_run = workflow_run_str(workflow_run);
            let sender = user_str(sender);
            format!("[{repo}:{branch}] Workflow run {wf} / {wf_run} requested by {sender}\n")
        }
    };
    Some(message)
}

/// `X-GitHub-Event: *`
fn default(_event_type: &str, _payload: Value) -> Option<String> {
    None
}

/// `user` -> `[user.login](user.html_url)`
fn user_str(user: &gh::User) -> String {
    let gh::User {
        login, html_url, ..
    } = user;
    format!("[{login}]({html_url})")
}

/// `repository` -> `[repository.full_name](repository.html_url)`
fn repo_str(repo: &gh::Repository) -> String {
    let gh::Repository {
        full_name,
        html_url,
        ..
    } = repo;
    format!("[{full_name}]({html_url})")
}

fn ser_ref_type(rt: &gh::CreateEventRefType) -> &str {
    match rt {
        gh::CreateEventRefType::Branch => "branch",
        gh::CreateEventRefType::Tag => "tag",
    }
}

/// `pr` -> `[#pr.number pr.title](pr.html_url)`
fn pr_str(pr: &gh::PullRequest) -> String {
    let gh::PullRequest {
        number,
        title,
        html_url,
        ..
    } = pr;
    format!("[#{number} {title}]({html_url})")
}

/// `[#issue.number issue.title](issue.html_url)`
fn issue_str(issue: &gh::Issue) -> String {
    let gh::Issue {
        number,
        title,
        html_url,
        ..
    } = issue;
    format!("[#{number} {title}]({html_url})")
}

/// `pr` -> `[#pr.number pr.title](pr.html_url)`
fn simple_pr_str(pr: &gh::SimplePullRequest) -> String {
    let gh::SimplePullRequest {
        number,
        title,
        html_url,
        ..
    } = pr;
    format!("[#{number} {title}]({html_url})")
}

/// `release` -> `[release.name](release.html_url)`
fn release_str(release: &gh::Release) -> String {
    let gh::Release { name, html_url, .. } = release;
    format!("[{name}]({html_url})")
}

/// `workflow` -> `[workflow.name](workflow.html_url)`
fn workflow_str(workflow: &gh::Workflow) -> String {
    let gh::Workflow { name, html_url, .. } = workflow;
    format!("[{name}]({html_url})")
}

/// `workflow_run` -> `[workflow_run.diplay_title](worlflow_run.html_url)`
fn workflow_run_str(workflow_run: &gh::WorkflowRun) -> String {
    let gh::WorkflowRun {
        display_title,
        html_url,
        ..
    } = workflow_run;
    format!("[{display_title}]({html_url})")
}

/// `workflow_job` -> `[workflow_job.workflow_name / workflow_job.name](workflow_job.html_url)`
fn workflow_job_str(workflow_job: &gh::WorkflowJob) -> String {
    let gh::WorkflowJob {
        workflow_name,
        name,
        html_url,
        ..
    } = workflow_job;
    match workflow_name.as_deref() {
        None => format!("[{name}]({html_url})"),
        Some(workflow_name) => format!("[{workflow_name} / {name}]({html_url})"),
    }
}

fn workflow_steps_str<'a, I>(steps: I) -> String
where
    I: IntoIterator<Item = &'a gh::WorkflowStep>,
{
    use gh::WorkflowStep::{Completed, InProgress, Queued};
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
                format!("{number}. `{name}`: completed as {conclusion}")
            }
            InProgress(s) => {
                let gh::WorkflowStepInProgress { number, name, .. } = s;
                format!("{number}. `{name}`: in progress")
            }
            Queued(s) => {
                let gh::WorkflowStepQueued { number, name, .. } = s;
                format!("{number}. `{name}`: queued")
            }
        })
        .join("\n")
}
