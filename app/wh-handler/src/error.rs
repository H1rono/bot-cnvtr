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
        use serde_json::error::Category;
        match value {
            Error::MissingField => domain::Error::BadRequest,
            Error::WrongType => domain::Error::BadRequest,
            Error::SerdeJson(e) => match e.classify() {
                Category::Data => domain::Error::BadRequest,
                Category::Io => domain::Error::Unexpected(e.into()),
                Category::Eof => domain::Error::BadRequest,
                Category::Syntax => domain::Error::BadRequest,
            },
        }
    }
}

pub type Result<T, E = Error> = ::std::result::Result<T, E>;
