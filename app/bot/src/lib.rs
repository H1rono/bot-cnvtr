use std::borrow::Cow;

use traq_bot_http::Event;

use domain::{Error, Infra};
use usecases::Bot;

pub(crate) mod cli;
mod messages;
mod system;

#[derive(Debug, Clone)]
pub struct BotImpl {
    pub name: String,
    pub id: String,
    pub user_id: String,
}

impl BotImpl {
    pub fn new<'a>(
        name: impl Into<Cow<'a, str>>,
        id: impl Into<Cow<'a, str>>,
        user_id: impl Into<Cow<'a, str>>,
    ) -> Self {
        let name = name.into().into_owned();
        let id = id.into().into_owned();
        let user_id = user_id.into().into_owned();
        Self { name, id, user_id }
    }
}

impl<I: Infra> Bot<I> for BotImpl
where
    Error: From<I::Error>,
{
    type Error = Error;

    #[tracing::instrument(skip_all, fields(event_kind = %event.kind()))]
    async fn handle_event(&self, infra: &I, event: Event) -> Result<(), Self::Error> {
        use Event::*;
        match event {
            Joined(payload) => self.on_joined(infra, payload).await,
            Left(payload) => self.on_left(infra, payload).await,
            MessageCreated(payload) => self.on_message_created(infra, payload).await,
            DirectMessageCreated(payload) => self.on_direct_message_created(infra, payload).await,
            _ => Ok(()),
        }
    }
}
