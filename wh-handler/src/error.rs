use thiserror::Error as ThisError;

use hyper::http::StatusCode;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("json field")]
    MissingField,
    #[error("wrong type")]
    WrongType,
    #[error("error while parsing payload: {0}")]
    SerdeJson(#[from] serde_json::Error),
}

impl From<Error> for StatusCode {
    fn from(value: Error) -> Self {
        use Error::*;
        match value {
            MissingField => StatusCode::BAD_REQUEST,
            WrongType => StatusCode::BAD_REQUEST,
            SerdeJson(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

pub type Result<T, E = Error> = ::std::result::Result<T, E>;
