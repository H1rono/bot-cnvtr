use std::{error::Error, fmt::Display};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub bot_id: String,
    pub bot_user_id: String,
    pub bot_access_token: String,
    pub verification_token: String,
}

#[derive(Debug)]
pub enum LoadError {
    DotEnvy(dotenvy::Error),
    Envy(envy::Error),
}

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

impl Config {
    pub fn from_env() -> Result<Self, LoadError> {
        dotenvy::from_filename(".env.dev")?;
        dotenvy::from_filename_override(".env")?;
        Ok(envy::from_env()?)
    }
}
