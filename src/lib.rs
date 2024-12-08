pub mod signal;
pub mod wrappers;

use std::time::Duration;

use serde::{Deserialize, Serialize};

#[must_use]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RouterConfig {}

#[must_use]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CronConfig {
    pub cron_period: String,
}

impl Default for CronConfig {
    fn default() -> Self {
        Self {
            cron_period: "10s".to_string(),
        }
    }
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

#[must_use]
#[derive(Debug, Clone)]
pub struct ConfigComposite {
    pub bot_config: wrappers::app::BotConfig,
    pub router_config: RouterConfig,
    pub client_config: wrappers::infra::TraqClientConfig,
    pub repo_config: wrappers::infra::RepoConfig,
    pub cron_config: CronConfig,
}

impl ConfigComposite {
    pub fn from_env() -> envy::Result<Self> {
        Ok(Self {
            bot_config: envy::from_env()?,
            router_config: envy::from_env()?,
            client_config: envy::from_env()?,
            repo_config: wrappers::infra::RepoConfig::from_env()?,
            cron_config: envy::from_env().unwrap_or_default(),
        })
    }
}
