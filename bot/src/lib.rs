use traq::apis::configuration::Configuration;
use traq_bot_http::Event;

use config::BotConfig;
use model::Database;

mod api;
mod error;
mod messages;
mod system;

pub use error::{Error, Result};

// pub trait Bot

#[derive(Debug, Clone)]
pub struct Bot {
    pub id: String,
    pub user_id: String,
    pub config: Configuration,
}

impl Bot {
    pub fn new(id: &str, user_id: &str, access_token: &str) -> Self {
        let id = id.to_string();
        let user_id = user_id.to_string();
        let config = Configuration {
            bearer_access_token: Some(access_token.to_string()),
            ..Default::default()
        };
        Self {
            id,
            user_id,
            config,
        }
    }

    pub fn from_config(bot_config: BotConfig) -> Self {
        let BotConfig {
            bot_id,
            bot_user_id,
            bot_access_token,
            ..
        } = bot_config;
        let config = Configuration {
            bearer_access_token: Some(bot_access_token),
            ..Default::default()
        };
        Self {
            id: bot_id,
            user_id: bot_user_id,
            config,
        }
    }

    pub async fn handle_event(&self, db: &Database, event: Event) -> Result<(), Error> {
        use Event::*;
        match event {
            Joined(payload) => self.on_joined(payload, db).await,
            Left(payload) => self.on_left(payload, db).await,
            MessageCreated(payload) => self.on_message_created(payload, db).await,
            DirectMessageCreated(payload) => self.on_direct_message_created(payload, db).await,
            _ => Ok(()),
        }
    }
}
