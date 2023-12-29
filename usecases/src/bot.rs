use std::future::Future;

use traq_bot_http::Event;

use domain::Infra;

use crate::config::Config;
use crate::error::{Error, Result};

pub trait Bot<I: Infra>: Send + Sync + 'static {
    type Error: Send + Sync + 'static;

    fn handle_event(
        &self,
        infra: &I,
        event: Event,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;
}

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

    pub async fn handle_event<E1, E2>(
        &self,
        repo: &impl domain::Repository<Error = E1>,
        client: &impl domain::TraqClient<Error = E2>,
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

impl<I: Infra> Bot<I> for BotImpl
where
    crate::Error: From<I::Error>,
{
    type Error = crate::Error;

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
