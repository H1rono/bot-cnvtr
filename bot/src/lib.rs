use traq_bot_http::Event;
use traq_client::Client;

use repository::AllRepository;

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
    pub client: Client,
}

impl Bot {
    pub fn new(id: &str, user_id: &str, access_token: &str) -> Self {
        let id = id.to_string();
        let user_id = user_id.to_string();
        let client = Client::new(access_token);
        Self {
            id,
            user_id,
            client,
        }
    }

    pub fn from_config(bot_config: Config) -> Self {
        let Config {
            bot_id,
            bot_user_id,
            bot_access_token,
            ..
        } = bot_config;
        let client_config = traq_client::Config { bot_access_token };
        let client = Client::from_config(client_config);
        Self {
            id: bot_id,
            user_id: bot_user_id,
            client,
        }
    }

    pub async fn handle_event(&self, repo: &impl AllRepository, event: Event) -> Result<(), Error> {
        use Event::*;
        match event {
            Joined(payload) => self.on_joined(payload, repo).await,
            Left(payload) => self.on_left(payload, repo).await,
            MessageCreated(payload) => self.on_message_created(payload, repo).await,
            DirectMessageCreated(payload) => self.on_direct_message_created(payload, repo).await,
            _ => Ok(()),
        }
    }
}
