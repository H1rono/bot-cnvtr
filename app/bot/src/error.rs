#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub enum Error {
    TraqBot(#[from] traq_bot_http::Error),
    Domain(#[from] domain::Error),
}

impl From<Error> for domain::Error {
    fn from(value: Error) -> Self {
        match value {
            Error::TraqBot(e) => domain::Error::Unexpected(e.into()),
            Error::Domain(e) => e,
        }
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
