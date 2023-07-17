use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::Value;

use super::{AppState, Error, Result};
use crate::model::Webhook;

/// GET /wh/:id
pub(super) async fn get_wh(
    State(st): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Webhook>> {
    st.db
        .find_webhook(&id)
        .await?
        .ok_or(Error::NotFound)
        .map(Json)
}

/// POST /wh/:id/github
pub(super) async fn wh_github(
    State(_st): State<AppState>,
    Path(_id): Path<String>,
    Json(_payload): Json<Value>,
) -> Result<StatusCode> {
    // TODO
    Ok(StatusCode::NOT_IMPLEMENTED)
}

/// POST /wh/:id/gitea
pub(super) async fn wh_gitea(
    State(_st): State<AppState>,
    Path(_id): Path<String>,
    Json(_payload): Json<Value>,
) -> Result<StatusCode> {
    // TODO
    Ok(StatusCode::NOT_IMPLEMENTED)
}

/// POST /wh/:id/clickup
pub(super) async fn wh_clickup(
    State(_st): State<AppState>,
    Path(_id): Path<String>,
    Json(_payload): Json<Value>,
) -> Result<StatusCode> {
    // TODO
    Ok(StatusCode::NOT_IMPLEMENTED)
}