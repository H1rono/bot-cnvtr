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

impl From<Error> for usecases::Error {
    fn from(value: Error) -> Self {
        match value {
            Error::MissingField => usecases::Error::BadRequest(None),
            Error::WrongType => usecases::Error::BadRequest(None),
            Error::SerdeJson(s) => usecases::Error::Serde(s),
        }
    }
}
