use reqwest::StatusCode;
use thiserror::Error as ThisError;

use traq::apis::configuration::Configuration;
use traq_bot_http::Event;

use config::BotConfig;
use model::Database;

mod api;
mod error;
mod messages;
mod system;

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

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("json parse failed")]
    Serde(#[from] serde_json::Error),
    #[error("io operation failed")]
    Io(#[from] std::io::Error),
    #[error("http reqest failed")]
    Reqwest(#[from] reqwest::Error),
    #[error("sqlx error")]
    Sqlx(#[from] sqlx::Error),
    #[error("got response with error code")]
    BadResponse { status: StatusCode, content: String },
}

pub type Result<T, E = Error> = std::result::Result<T, E>;