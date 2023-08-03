use serde::{Deserialize, Serialize};

mod error;

pub use error::{LoadError, Result};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BotConfig {
    pub bot_id: String,
    pub bot_user_id: String,
    pub bot_access_token: String,
    pub verification_token: String,
}

impl BotConfig {
    pub fn from_env() -> Result<Self> {
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
    pub fn from_env() -> Result<Self> {
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
    fn load_env() -> Result<Self> {
        let bot_c = BotConfig::from_env()?;
        let db_c = DbConfig::from_env()?;
        Ok(Self(bot_c, db_c))
    }

    pub fn from_env() -> Result<Self> {
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
