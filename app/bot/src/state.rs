use std::sync::Arc;

use traq_bot_http::payloads;

use domain::Infra;

use super::State;
use crate::error::Error;

impl<I: Infra> State<I> {
    pub(crate) async fn on_joined(
        (state, payload): (Arc<Self>, payloads::JoinedPayload),
    ) -> Result<(), Error> {
        state.bot.on_joined(&*state.infra, payload).await
    }

    pub(crate) async fn on_left(
        (state, payload): (Arc<Self>, payloads::LeftPayload),
    ) -> Result<(), Error> {
        state.bot.on_left(&*state.infra, payload).await
    }

    pub(crate) async fn on_message_created(
        (state, payload): (Arc<Self>, payloads::MessageCreatedPayload),
    ) -> Result<(), Error> {
        state.bot.on_message_created(&*state.infra, payload).await
    }

    pub(crate) async fn on_direct_message_created(
        (state, payload): (Arc<Self>, payloads::DirectMessageCreatedPayload),
    ) -> Result<(), Error> {
        state
            .bot
            .on_direct_message_created(&*state.infra, payload)
            .await
    }
}
