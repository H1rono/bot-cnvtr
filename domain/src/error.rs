#[must_use]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("ill-formatted request")]
    BadRequest,
    #[error("unauthorized")]
    Unauthorized,
    #[error("resource not found")]
    NotFound,
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
    #[error("unimplemented")]
    NotImplemented,
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
