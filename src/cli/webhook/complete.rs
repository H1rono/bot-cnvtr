use traq_bot_http::payloads::{DirectMessageCreatedPayload, MessageCreatedPayload};
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

impl From<MessageCreatedPayload> for WebhookList {
    fn from(value: MessageCreatedPayload) -> Self {
        Self {
            user_id: value.message.user.id,
        }
    }
}

impl From<DirectMessageCreatedPayload> for WebhookList {
    fn from(value: DirectMessageCreatedPayload) -> Self {
        Self {
            user_id: value.message.user.id,
        }
    }
}

#[derive(Debug, Clone)]
pub struct WebhookDelete {
    pub webhook_id: String,
}
