use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error as ThisError;
use wh_handler::Error as WebhookHandlerError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("not found")]
    NotFound,
    #[error("bad request")]
    BadRequest,
    #[error("sqlx error")]
    Sqlx(#[from] sqlx::Error),
    #[error("unexpected error")]
    Unexpected, // TODO: anyhow
}

impl From<usecases::Error> for Error {
    fn from(value: usecases::Error) -> Self {
        match value {
            usecases::Error::BadRequest(_) => Error::BadRequest,
            usecases::Error::Sqlx(e) => Error::Sqlx(e),
            usecases::Error::Serde(e) => {
                eprintln!("serde error: {}", e);
                Error::Unexpected
            }
            usecases::Error::Other(e) => {
                eprintln!("unexpected error: {}", e);
                Error::Unexpected
            }
        }
    }
}

impl From<WebhookHandlerError> for Error {
    fn from(value: WebhookHandlerError) -> Self {
        use WebhookHandlerError::*;
        match value {
            MissingField => Error::BadRequest,
            WrongType => Error::BadRequest,
            SerdeJson(_) => Error::Unexpected,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::NotFound => (StatusCode::NOT_FOUND, "Not Found").into_response(),
            Self::BadRequest => (StatusCode::BAD_REQUEST, "Bad Request").into_response(),
            Self::Sqlx(e) => {
                eprintln!("sqlx error: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
            Self::Unexpected => {
                eprintln!("unexpected error");
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
