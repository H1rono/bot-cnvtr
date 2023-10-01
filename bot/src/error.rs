use reqwest::StatusCode;
use thiserror::Error as ThisError;

use traq::apis::Error as ApiError;
use traq_client::Error as ClientError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("json parse failed")]
    Serde(#[from] serde_json::Error),
    #[error("io operation failed")]
    Io(#[from] std::io::Error),
    #[error("http reqest failed")]
    Reqwest(#[from] reqwest::Error),
    #[error("sqlx error")]
    Sqlx(#[from] sqlx::Error),
    #[error("traq-client error: {0}")]
    Client(#[from] ClientError),
    #[error("got response with error code")]
    BadResponse { status: StatusCode, content: String },
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

impl<T> From<ApiError<T>> for Error {
    fn from(value: ApiError<T>) -> Self {
        use ApiError::*;
        match value {
            Reqwest(e) => e.into(),
            Serde(e) => e.into(),
            Io(e) => e.into(),
            ResponseError(e) => Self::BadResponse {
                status: e.status,
                content: e.content,
            },
        }
    }
}
