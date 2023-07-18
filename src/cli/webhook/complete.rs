use traq_bot_http::payloads::{types::Message, DirectMessageCreatedPayload, MessageCreatedPayload};
use uuid::Uuid;

use crate::model::Owner;

#[derive(Debug, Clone)]
pub enum Webhook {
    Create(WebhookCreate),
    List(WebhookList),
    Delete(WebhookDelete),
}

#[derive(Debug, Clone)]
pub struct WebhookCreate {
    pub channel_name: Option<String>,
    pub channel_id: Uuid,
    pub owner: Owner,
}

#[derive(Debug, Clone)]
pub struct WebhookList {
    pub user_id: Uuid,
}

impl From<Message> for WebhookList {
    fn from(value: Message) -> Self {
        Self {
            user_id: value.user.id,
        }
    }
}

impl From<MessageCreatedPayload> for WebhookList {
    fn from(value: MessageCreatedPayload) -> Self {
        value.message.into()
    }
}

impl From<DirectMessageCreatedPayload> for WebhookList {
    fn from(value: DirectMessageCreatedPayload) -> Self {
        value.message.into()
    }
}

#[derive(Debug, Clone)]
pub struct WebhookDelete {
    pub webhook_id: String,
}
