use traq_bot_http::Event;

use repository::AllRepository;
use traq_client::Client;

mod config;
mod error;
mod messages;
mod system;

pub use config::Config;
pub use error::{Error, Result};

// pub trait Bot

#[derive(Debug, Clone)]
pub struct Bot {
    pub id: String,
    pub user_id: String,
}

impl Bot {
    pub fn new(id: &str, user_id: &str) -> Self {
        let id = id.to_string();
        let user_id = user_id.to_string();
        Self { id, user_id }
    }

    pub fn from_config(bot_config: Config) -> Self {
        let Config {
            bot_id,
            bot_user_id,
            ..
        } = bot_config;
        Self {
            id: bot_id,
            user_id: bot_user_id,
        }
    }

    pub async fn handle_event(
        &self,
        client: &impl Client,
        repo: &impl AllRepository,
        event: Event,
    ) -> Result<(), Error> {
        use Event::*;
        match event {
            Joined(payload) => self.on_joined(client, repo, payload).await,
            Left(payload) => self.on_left(client, repo, payload).await,
            MessageCreated(payload) => self.on_message_created(client, repo, payload).await,
            DirectMessageCreated(payload) => {
                self.on_direct_message_created(client, repo, payload).await
            }
            _ => Ok(()),
        }
    }
}
