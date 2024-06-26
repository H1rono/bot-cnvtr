use reqwest::StatusCode;
use thiserror::Error as ThisError;
use traq::apis::Error as ApiError;

#[must_use]
#[derive(Debug, ThisError)]
pub enum Error {
    #[error("json parse failed: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("io operation failed: {0}")]
    Io(#[from] std::io::Error),
    #[error("http reqest failed: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("got response with error code: {}", .status.as_str())]
    BadResponse { status: StatusCode, content: String },
}

impl<T> From<ApiError<T>> for Error {
    fn from(value: ApiError<T>) -> Self {
        use ApiError::{Io, Reqwest, ResponseError, Serde};
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

impl From<Error> for domain::Error {
    fn from(value: Error) -> Self {
        domain::Error::Unexpected(value.into())
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
