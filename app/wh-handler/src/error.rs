use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("json field")]
    MissingField,
    #[error("wrong type")]
    WrongType,
    #[error("error while parsing payload: {0}")]
    SerdeJson(#[from] serde_json::Error),
}

impl From<Error> for domain::Error {
    fn from(value: Error) -> Self {
        match value {
            Error::MissingField => domain::Error::BadRequest,
            Error::WrongType => domain::Error::BadRequest,
            e => domain::Error::Unexpected(e.into()),
        }
    }
}

pub type Result<T, E = Error> = ::std::result::Result<T, E>;
