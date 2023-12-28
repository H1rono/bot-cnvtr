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

pub type Result<T, E = Error> = ::std::result::Result<T, E>;
