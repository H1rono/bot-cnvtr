use async_trait::async_trait;
use axum::{
    extract::{FromRequestParts, Path, State},
    response::IntoResponse,
    Json,
};
use http::{request::Parts, HeaderMap, StatusCode};
use tracing::{info, instrument};

use domain::{Repository, Webhook, WebhookId};
use usecases::WebhookHandler;

use crate::{
    error::{Error, Result},
    AppState,
};

#[derive(Debug, Clone)]
pub struct Wh(pub Webhook);

#[derive(Debug, thiserror::Error)]
pub enum WhRejection {
    #[error(transparent)]
    Path(#[from] axum::extract::rejection::PathRejection),
    #[error(transparent)]
    Logic(#[from] Error),
}

impl IntoResponse for WhRejection {
    fn into_response(self) -> axum::response::Response {
        match self {
            WhRejection::Path(p) => p.into_response(),
            WhRejection::Logic(l) => l.into_response(),
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Wh
where
    S: AppState<Error = domain::Error>,
{
    type Rejection = WhRejection;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Path(id) = Path::<WebhookId>::from_request_parts(parts, state).await?;
        let webhook = state
            .repo()
            .find_webhook(&id)
            .await
            .map_err(domain::Error::from)
            .map_err(Error::from)?
            .ok_or(Error::from(domain::Error::NotFound))?;
        Ok(Wh(webhook))
    }
}

/// GET /wh/:id
#[instrument(skip_all, fields(webhook_id = %webhook.id))]
pub(super) async fn get_wh<S>(Wh(webhook): Wh) -> Json<Webhook>
where
    S: AppState<Error = domain::Error>,
{
    info!("GET webhook info");
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
    S: AppState<Error = domain::Error>,
{
    info!("POST github webhook");
    let infra = st.infra();
    st.webhook_handler()
        .github_webhook(infra, webhook, headers, &payload)
        .await?;
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
    S: AppState<Error = domain::Error>,
{
    info!("POST gitea webhook");
    let infra = st.infra();
    st.webhook_handler()
        .gitea_webhook(infra, webhook, headers, &payload)
        .await?;
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
    S: AppState<Error = domain::Error>,
{
    info!("POST clickup webhook");
    let infra = st.infra();
    st.webhook_handler()
        .clickup_webhook(infra, webhook, headers, &payload)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
