use std::str::from_utf8;

use http::HeaderMap;
use indoc::formatdoc;
use serde_json::Value;
use teahook as th;

use domain::{Error, Event, EventSubscriber, Infra, Webhook};

use super::utils::{extract_header_value, OptionExt};
use crate::WebhookHandlerImpl;

impl WebhookHandlerImpl {
    pub(crate) async fn handle_gitea<I>(
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
        let kind = "gitea".to_string().into(); // TODO: event_type
        let event = Event {
            channel_id: webhook.channel_id,
            kind,
            body: message.into(),
        };
        subscriber.send(event).await?;
        Ok(())
    }
}

#[tracing::instrument(target = "wh_handler::gitea::handle", skip_all)]
fn handle(headers: HeaderMap, payload: &str) -> Result<Option<String>, Error> {
    macro_rules! match_event {
        ($t:expr => $p:expr; $($i:ident),* ; default = [ $($di:ident),* ]) => {{
            let local_event_type = $t;
            match local_event_type {
                $(stringify!($i) => {
                    tracing::info!("X-Gitea-Event: {local_event_type}");
                    Some($i(from_str($p).map_err(anyhow::Error::from)?)?)
                })*
                $(stringify!($di))|* => {
                    tracing::info!("X-Gitea-Event: {local_event_type}");
                    default($t, from_str($p).map_err(anyhow::Error::from)?)
                }
                ut => {
                    tracing::warn!("unexpected event: `X-Gitea-Event: {ut}`");
                    return Err(Error::BadRequest);
                }
            }}
        };
    }

    use serde_json::from_str;
    // https://github.com/traPtitech/gitea/blob/8abe54a9d4db1fdce7c517dc500a51e77d1f2c16/services/webhook/deliver.go#L124-L138
    // https://github.com/traPtitech/gitea/blob/8abe54a9d4db1fdce7c517dc500a51e77d1f2c16/modules/webhook/type.go#L11-L33
    let event_type = extract_header_value(&headers, "X-Gitea-Event")
        .and_then(|v| from_utf8(v).map_err(|_| Error::BadRequest))?;
    let message = match_event!(
        event_type => payload;
        create, delete, fork, push, issues, pull_request;
        default = [
            issue_assign, issue_label, issue_milestone, issue_comment,
            pull_request_assign, pull_request_label, pull_request_milestone,
            pull_request_comment, pull_request_review_approved, pull_request_review_rejected,
            pull_request_revew_comment, pull_request_sync, pull_request_review_request, wiki,
            repository, release, package
        ]
    );
    let Some(message) = message else {
        return Ok(None);
    };
    Ok(Some(message))
}

macro_rules! unwrap_opt_boxed {
    ($i:ident) => {
        let $i = $i.as_deref().ok_or_err()?;
    };

    ($($i:ident),*) => {
        $(let $i = $i.as_deref().ok_or_err()?;)*
    };
}

/// `X-Gitea-Event: create`
fn create(payload: th::CreatePayload) -> Result<String, Error> {
    let th::CreatePayload {
        r#ref,
        ref_type,
        repo,
        sender,
        ..
    } = &payload;
    unwrap_opt_boxed! {repo, sender}
    let message = format!(
        "[{repo}] {ref_type} `{ref}` was created by {sender}\n",
        repo = repo_str(repo),
        sender = user_str(sender)
    );
    Ok(message)
}

/// `X-Gitea-Event: delete`
fn delete(payload: th::DeletePayload) -> Result<String, Error> {
    let th::DeletePayload {
        r#ref,
        ref_type,
        repo,
        sender,
        ..
    } = &payload;
    unwrap_opt_boxed! {repo, sender}
    let message = format!(
        "[{repo}] {ref_type} `{ref}` was deleted by {sender}\n",
        repo = repo_str(repo),
        sender = user_str(sender)
    );
    Ok(message)
}

/// `X-Gitea-Event: fork`
fn fork(payload: th::ForkPayload) -> Result<String, Error> {
    let th::ForkPayload {
        forkee,
        repo,
        sender,
    } = &payload;
    unwrap_opt_boxed! {repo, sender, forkee}
    let message = format!(
        "[{repo}] forked to {forkee} by {sender}\n",
        repo = repo_str(repo),
        forkee = repo_str(forkee),
        sender = user_str(sender),
    );
    Ok(message)
}

/// `X-Gitea-Event: push`
fn push(payload: th::PushPayload) -> Result<String, Error> {
    let th::PushPayload {
        r#ref,
        commits,
        repo,
        sender,
        ..
    } = &payload;
    unwrap_opt_boxed! {repo, sender, commits}
    let commit_count = commits.len();
    let commit_unit = if commit_count == 1 { "" } else { "s" };
    // githubと順序を合わせるためにrev
    let commits = commits
        .iter()
        .rev()
        .map(|c| {
            unwrap_opt_boxed! {c}
            let th::PayloadCommit {
                id, message, url, ..
            } = c;
            let id = &id[0..7];
            let message = message.lines().next().unwrap().trim_end();
            Ok(format!("[`{id}`]({url}) {message}"))
        })
        .collect::<Result<Vec<_>, Error>>()?
        .join("\n");
    Ok(formatdoc! {
        r#"
            [{repo}:{ref}] {commit_count} commit{commit_unit} was pushed by {sender}
            {commits}
        "#,
        repo = repo_str(repo),
        sender = user_str(sender)
    })
}

/// `X-Gitea-Event: issues`
fn issues(payload: th::IssuePayload) -> Result<String, Error> {
    let th::IssuePayload {
        action,
        index,
        issue,
        repository: repo,
        sender,
        ..
    } = &payload;
    unwrap_opt_boxed! {repo, sender, issue}
    let message = format!(
        "[{repo}] issue [#{index} {title}]({html_url}) {action} by {sender}\n",
        repo = repo_str(repo),
        title = &issue.title,
        html_url = &issue.html_url,
        sender = user_str(sender)
    );
    Ok(message)
}

// `X-Gitea-Event: pull_request`
fn pull_request(payload: th::PullRequestPayload) -> Result<String, Error> {
    let th::PullRequestPayload {
        action,
        pull_request,
        repository: repo,
        sender,
        ..
    } = &payload;
    unwrap_opt_boxed! {repo, sender, pull_request}
    let message = format!(
        "[{repo}] Pull Request {pr} {action} by {sender}\n",
        repo = repo_str(repo),
        pr = pr_str(pull_request),
        sender = user_str(sender)
    );
    Ok(message)
}

/// `X-Gitea-Event: *`
fn default(_event_type: &str, _payload: Value) -> Option<String> {
    None
}

fn repo_str(repo: &th::Repository) -> String {
    let th::Repository {
        full_name,
        html_url,
        ..
    } = repo;
    format!("[{full_name}]({html_url})")
}

fn user_str(user: &th::User) -> String {
    let th::User {
        user_name,
        avatar_url,
        ..
    } = user;
    format!("[{user_name}]({avatar_url})")
}

fn pr_str(pr: &th::PullRequest) -> String {
    let th::PullRequest {
        id,
        title,
        html_url,
        ..
    } = pr;
    format!("[#{id} {title}]({html_url})")
}
