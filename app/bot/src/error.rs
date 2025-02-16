use std::{error, fmt};

pub struct Error(pub domain::Failure);

impl From<anyhow::Error> for Error {
    fn from(value: anyhow::Error) -> Self {
        domain::Failure::from(value).into()
    }
}

impl From<domain::Failure> for Error {
    fn from(value: domain::Failure) -> Self {
        Error(value)
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl error::Error for Error {}
