use serde::{Deserialize, Serialize};
use sqlx::mysql::{MySqlConnectOptions, MySqlPool};

use crate::RepositoryImpl;

#[must_use]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Opt {
    pub hostname: String,
    pub user: String,
    pub password: String,
    pub port: u16,
    pub database: String,
}

impl Opt {
    pub async fn connect(self) -> anyhow::Result<RepositoryImpl> {
        let Opt {
            hostname,
            user,
            password,
            port,
            database,
        } = &self;
        let opt = MySqlConnectOptions::new()
            .host(hostname)
            .username(user)
            .password(password)
            .port(*port)
            .database(database);
        let pool = MySqlPool::connect_with(opt).await?;
        Ok(RepositoryImpl::new(pool))
    }
}
