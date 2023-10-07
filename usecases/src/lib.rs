use traq_bot_http::Event;

pub(crate) mod cli;
mod config;
mod error;
mod messages;
mod system;
mod traits;

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
        } = bot_config;
        Self {
            id: bot_id,
            user_id: bot_user_id,
        }
    }

    pub async fn handle_event<E1, E2>(
        &self,
        repo: &impl traits::Repository<Error = E1>,
        client: &impl traits::TraqClient<Error = E2>,
        event: Event,
    ) -> Result<(), Error>
    where
        Error: From<E1> + From<E2>,
    {
        use Event::*;
        match event {
            Joined(payload) => self.on_joined(repo, client, payload).await,
            Left(payload) => self.on_left(repo, client, payload).await,
            MessageCreated(payload) => self.on_message_created(repo, client, payload).await,
            DirectMessageCreated(payload) => {
                self.on_direct_message_created(repo, client, payload).await
            }
            _ => Ok(()),
        }
    }
}
