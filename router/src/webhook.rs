use axum::{
    extract::{FromRequestParts, Path, State},
    response::IntoResponse,
    Json,
};
use http::{request::Parts, HeaderMap, StatusCode};
use tracing::{debug, instrument, warn};

use domain::{Infra, Repository, Webhook, WebhookId};
use usecases::{WebhookHandler, WebhookKind};

use crate::{
    error::{Error, Result},
    AppState,
};

#[must_use]
#[derive(Debug, Clone)]
pub struct Wh(pub Webhook);

#[must_use]
#[derive(Debug, thiserror::Error)]
pub enum WhRejection {
    #[error(transparent)]
    Path(#[from] axum::extract::rejection::PathRejection),
    #[error(transparent)]
    Logic(#[from] Error),
}

impl From<domain::Failure> for WhRejection {
    fn from(value: domain::Failure) -> Self {
        Error::from(value).into()
    }
}

impl IntoResponse for WhRejection {
    fn into_response(self) -> axum::response::Response {
        match self {
            WhRejection::Path(p) => p.into_response(),
            WhRejection::Logic(l) => l.into_response(),
        }
    }
}

impl<S> FromRequestParts<S> for Wh
where
    S: AppState,
{
    type Rejection = WhRejection;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Path(id) = Path::<WebhookId>::from_request_parts(parts, state).await?;
        let webhook = state.infra().repo().find_webhook(&id).await?;
        Ok(Wh(webhook))
    }
}

/// GET /wh/:id
#[instrument(skip_all, fields(webhook_id = %webhook.id))]
pub(super) async fn get_wh<S>(Wh(webhook): Wh) -> Json<Webhook>
where
    S: AppState,
{
    debug!("GET webhook info");
    Json(webhook)
}

/// POST /wh/:id/github
#[instrument(skip_all, fields(webhook_id = %webhook.id))]
pub(super) async fn wh_github<S>(
    State(st): State<S>,
    Wh(webhook): Wh,
    headers: HeaderMap,
    payload: String,
) -> Result<StatusCode>
where
    S: AppState,
{
    debug!("POST github webhook");
    let infra = st.infra();
    st.webhook_handler()
        .handle(WebhookKind::GitHub, infra, webhook, headers, &payload)
        .await
        .inspect_err(|e| warn!("{e}"))?;
    Ok(StatusCode::NO_CONTENT)
}

/// POST /wh/:id/gitea
#[instrument(skip_all, fields(webhook_id = %webhook.id))]
pub(super) async fn wh_gitea<S>(
    State(st): State<S>,
    Wh(webhook): Wh,
    headers: HeaderMap,
    payload: String,
) -> Result<StatusCode>
where
    S: AppState,
{
    debug!("POST gitea webhook");
    let infra = st.infra();
    st.webhook_handler()
        .handle(WebhookKind::Gitea, infra, webhook, headers, &payload)
        .await
        .inspect_err(|e| warn!("{e}"))?;
    Ok(StatusCode::NO_CONTENT)
}

/// POST /wh/:id/clickup
#[instrument(skip_all, fields(webhook_id = %webhook.id))]
pub(super) async fn wh_clickup<S>(
    State(st): State<S>,
    Wh(webhook): Wh,
    headers: HeaderMap,
    payload: String,
) -> Result<StatusCode>
where
    S: AppState,
{
    debug!("POST clickup webhook");
    let infra = st.infra();
    st.webhook_handler()
        .handle(WebhookKind::Clickup, infra, webhook, headers, &payload)
        .await
        .inspect_err(|e| warn!("{e}"))?;
    Ok(StatusCode::NO_CONTENT)
}
