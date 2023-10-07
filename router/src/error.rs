use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error as ThisError;
use traq_client::Error as ClientError;
use wh_handler::Error as WebhookHandlerError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("not found")]
    NotFound,
    #[error("bad request")]
    BadRequest,
    #[error("sqlx error")]
    Sqlx(#[from] sqlx::Error),
    #[error("traq-client error: {0}")]
    Client(#[from] ClientError),
    #[error("processing error")]
    Process(#[from] usecases::Error),
    #[error("unexpected error")]
    Unexpected,
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
            Self::Process(e) => {
                eprintln!("processing error: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
            Self::Client(e) => {
                eprintln!("client error: {}", e);
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
