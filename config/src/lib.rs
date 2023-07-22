use std::{error::Error, fmt::Display};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BotConfig {
    pub bot_id: String,
    pub bot_user_id: String,
    pub bot_access_token: String,
    pub verification_token: String,
}

impl BotConfig {
    pub fn from_env() -> Result<Self, LoadError> {
        envy::from_env().map_err(LoadError::Envy)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DbConfig {
    pub database: String,
    pub hostname: String,
    pub password: String,
    pub port: String,
    pub user: String,
}

impl DbConfig {
    pub fn from_env() -> Result<Self, LoadError> {
        envy::prefixed("NS_MARIADB_")
            .from_env()
            .or_else(|_| envy::prefixed("MYSQL_").from_env())
            .map_err(LoadError::Envy)
    }

    pub fn database_url(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}/{}",
            self.user, self.password, self.hostname, self.port, self.database
        )
    }
}

pub struct Config(pub BotConfig, pub DbConfig);

impl Config {
    fn load_env() -> Result<Self, LoadError> {
        let bot_c = BotConfig::from_env()?;
        let db_c = DbConfig::from_env()?;
        Ok(Self(bot_c, db_c))
    }

    pub fn from_env() -> Result<Self, LoadError> {
        Self::load_env()
            .or_else(|_| {
                dotenvy::from_filename_override(".env")?;
                Self::load_env()
            })
            .or_else(|_| {
                dotenvy::from_filename_override(".env.dev")?;
                Self::load_env()
            })
    }
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
