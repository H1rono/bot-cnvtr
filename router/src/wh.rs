use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    Json,
};
use serde_json::Value;

use domain::{Infra, Repository, TraqClient};
use domain::{Webhook, WebhookId};
use usecases::{App, WebhookHandler};

use super::{AppState, AppStateImpl, Error, Result};

/// GET /wh/:id
pub(super) async fn get_wh<I, A>(
    State(st): State<AppStateImpl<I, A>>,
    Path(id): Path<WebhookId>,
) -> Result<Json<Webhook>>
where
    I: Infra<Error = usecases::Error>,
    A: App<I, Error = usecases::Error>,
{
    let repo = st.infra.repo();
    repo.find_webhook(&id)
        .await
        .map_err(usecases::Error::from)?
        .ok_or(Error::NotFound)
        .map(Json)
}

/// POST /wh/:id/github
pub(super) async fn wh_github<I, A>(
    State(st): State<AppStateImpl<I, A>>,
    Path(id): Path<WebhookId>,
    headers: HeaderMap,
    Json(payload): Json<Value>,
) -> Result<StatusCode>
where
    I: Infra<Error = usecases::Error>,
    A: App<I, Error = usecases::Error>,
{
    let client = st.infra.traq_client();
    let repo = st.infra.repo();
    let webhook = repo
        .find_webhook(&id)
        .await
        .map_err(usecases::Error::from)?
        .ok_or(Error::NotFound)?;
    let message = st
        .webhook_handler()
        .github_webhook(headers.iter(), payload)
        .map_err(usecases::Error::from)?;
    if message.is_none() {
        return Ok(StatusCode::NO_CONTENT);
    }
    let message = message.unwrap();
    client
        .send_message(&webhook.channel_id, message.trim(), false)
        .await
        .map_err(usecases::Error::from)?;
    Ok(StatusCode::NO_CONTENT)
}

/// POST /wh/:id/gitea
pub(super) async fn wh_gitea<I, A>(
    State(st): State<AppStateImpl<I, A>>,
    Path(id): Path<WebhookId>,
    headers: HeaderMap,
    Json(payload): Json<Value>,
) -> Result<StatusCode>
where
    I: Infra<Error = usecases::Error>,
    A: App<I, Error = usecases::Error>,
{
    let client = st.infra.traq_client();
    let repo = st.infra.repo();
    let webhook = repo
        .find_webhook(&id)
        .await
        .map_err(usecases::Error::from)?
        .ok_or(Error::NotFound)?;
    let message = st
        .webhook_handler()
        .gitea_webhook(headers.iter(), payload)
        .map_err(usecases::Error::from)?;
    if message.is_none() {
        return Ok(StatusCode::NO_CONTENT);
    }
    let message = message.unwrap();
    client
        .send_message(&webhook.channel_id, message.trim(), false)
        .await
        .map_err(usecases::Error::from)?;
    Ok(StatusCode::NOT_IMPLEMENTED)
}

/// POST /wh/:id/clickup
pub(super) async fn wh_clickup<I, A>(
    State(st): State<AppStateImpl<I, A>>,
    Path(id): Path<WebhookId>,
    headers: HeaderMap,
    Json(payload): Json<Value>,
) -> Result<StatusCode>
where
    I: Infra<Error = usecases::Error>,
    A: App<I, Error = usecases::Error>,
{
    let client = st.infra.traq_client();
    let repo = st.infra.repo();
    let webhook = repo
        .find_webhook(&id)
        .await
        .map_err(usecases::Error::from)?
        .ok_or(Error::NotFound)?;
    let message = st
        .webhook_handler()
        .clickup_webhook(headers.iter(), payload)
        .map_err(usecases::Error::from)?;
    if message.is_none() {
        return Ok(StatusCode::NO_CONTENT);
    }
    let message = message.unwrap();
    client
        .send_message(&webhook.channel_id, message.trim(), false)
        .await
        .map_err(usecases::Error::from)?;
    Ok(StatusCode::NOT_IMPLEMENTED)
}
