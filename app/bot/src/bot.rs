use traq_bot_http::Event;

use domain::{Error, Infra};
use usecases::Bot;

use crate::config::Config;

#[derive(Debug, Clone)]
pub struct BotImpl {
    pub id: String,
    pub user_id: String,
}

impl BotImpl {
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
}

impl<I: Infra> Bot<I> for BotImpl
where
    Error: From<I::Error>,
{
    type Error = Error;

    async fn handle_event(&self, infra: &I, event: Event) -> Result<(), Self::Error> {
        use Event::*;
        match event {
            Joined(payload) => {
                self.on_joined(infra.repo(), infra.traq_client(), payload)
                    .await
            }
            Left(payload) => {
                self.on_left(infra.repo(), infra.traq_client(), payload)
                    .await
            }
            MessageCreated(payload) => {
                self.on_message_created(infra.repo(), infra.traq_client(), payload)
                    .await
            }
            DirectMessageCreated(payload) => {
                self.on_direct_message_created(infra.repo(), infra.traq_client(), payload)
                    .await
            }
            _ => Ok(()),
        }
    }
}
