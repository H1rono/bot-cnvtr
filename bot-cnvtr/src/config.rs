use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BotConfig {
    pub bot_id: String,
    pub bot_user_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RepoConfig {
    pub database: String,
    pub hostname: String,
    pub password: String,
    pub port: String,
    pub user: String,
}

impl RepoConfig {
    pub fn from_env() -> envy::Result<Self> {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraqClientConfig {
    pub bot_access_token: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RouterConfig {
    pub verification_token: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CronConfig {
    pub cron_period: String,
}

impl TryFrom<CronConfig> for Duration {
    type Error = anyhow::Error;

    fn try_from(value: CronConfig) -> Result<Self, Self::Error> {
        let period = value.cron_period;
        if let Some(millis) = period.strip_suffix("ms") {
            return Ok(Duration::from_millis(millis.parse()?));
        }
        if let Some(secs) = period.strip_suffix('s') {
            return Ok(Duration::from_secs(secs.parse()?));
        }
        Err(anyhow::anyhow!("unexpected cron period: {}", period))
    }
}

pub struct ConfigComposite {
    pub bot_config: BotConfig,
    pub router_config: RouterConfig,
    pub client_config: TraqClientConfig,
    pub repo_config: RepoConfig,
    pub cron_config: CronConfig,
}

impl ConfigComposite {
    pub fn from_env() -> envy::Result<Self> {
        Ok(Self {
            bot_config: envy::from_env()?,
            router_config: envy::from_env()?,
            client_config: envy::from_env()?,
            repo_config: RepoConfig::from_env()?,
            cron_config: envy::from_env()?,
        })
    }
}
