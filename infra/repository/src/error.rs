#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct Error(#[from] pub sqlx::Error);

impl From<Error> for domain::Error {
    fn from(value: Error) -> Self {
        domain::Error::Unexpected(value.into())
    }
}

pub type Result<T, E = Error> = ::std::result::Result<T, E>;
