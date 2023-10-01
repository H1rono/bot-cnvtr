use reqwest::StatusCode;
use thiserror::Error as ThisError;
use traq::apis::Error as ApiError;

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

pub type Result<T, E = Error> = std::result::Result<T, E>;
