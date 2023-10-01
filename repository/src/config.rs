use envy::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub database: String,
    pub hostname: String,
    pub password: String,
    pub port: String,
    pub user: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        envy::prefixed("NS_MARIADB_")
            .from_env()
            .or_else(|_| envy::prefixed("MYSQL_").from_env())
    }

    pub fn database_url(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}/{}",
            self.user, self.password, self.hostname, self.port, self.database
        )
    }
}
