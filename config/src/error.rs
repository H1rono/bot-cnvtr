use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum LoadError {
    DotEnvy(dotenvy::Error),
    Envy(envy::Error),
}

pub type Result<T, E = LoadError> = std::result::Result<T, E>;

impl Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use LoadError::*;
        match self {
            DotEnvy(err) => write!(f, "LoadError::DotEnvy: {}", err),
            Envy(err) => write!(f, "LoadError::Envy: {}", err),
        }
    }
}

impl Error for LoadError {}

impl From<dotenvy::Error> for LoadError {
    fn from(value: dotenvy::Error) -> Self {
        LoadError::DotEnvy(value)
    }
}

impl From<envy::Error> for LoadError {
    fn from(value: envy::Error) -> Self {
        LoadError::Envy(value)
    }
}
