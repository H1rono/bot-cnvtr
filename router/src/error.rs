use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error as ThisError;

#[must_use]
#[derive(Debug, ThisError)]
#[error(transparent)]
pub struct Error(#[from] pub domain::Error);

impl IntoResponse for Error {
    #[tracing::instrument(skip_all, target = "router::error::Error::into_response")]
    fn into_response(self) -> axum::response::Response {
        use domain::Error as DE;
        match self.0 {
            DE::BadRequest => StatusCode::BAD_REQUEST.into_response(),
            DE::NotFound => StatusCode::NOT_FOUND.into_response(),
            DE::NotImplemented => StatusCode::NOT_IMPLEMENTED.into_response(),
            DE::Unauthorized => StatusCode::UNAUTHORIZED.into_response(),
            DE::Unexpected(e) => {
                tracing::error!("Unexpected error while routing: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
