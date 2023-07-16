use traq::apis::configuration::Configuration;

use super::config::BotConfig;

#[derive(Debug, Clone)]
pub struct Bot {
    pub id: String,
    pub user_id: String,
    pub client: Configuration,
}

impl Bot {
    pub fn new(id: &str, user_id: &str, access_token: &str) -> Self {
        let id = id.to_string();
        let user_id = user_id.to_string();
        let client = Configuration {
            bearer_access_token: Some(access_token.to_string()),
            ..Default::default()
        };
        Self {
            id,
            user_id,
            client,
        }
    }

    pub fn from_config(bot_config: BotConfig) -> Self {
        let BotConfig {
            bot_id,
            bot_user_id,
            bot_access_token,
            ..
        } = bot_config;
        let client = Configuration {
            bearer_access_token: Some(bot_access_token),
            ..Default::default()
        };
        Self {
            id: bot_id,
            user_id: bot_user_id,
            client,
        }
    }
}
