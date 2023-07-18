use serde::{Deserialize, Serialize};
use traq_bot_http::payloads::{types::Message, DirectMessageCreatedPayload, MessageCreatedPayload};
use uuid::Uuid;

use super::incomplete;
use crate::{cli::Completed, model::Owner};

#[derive(Debug, Clone)]
pub enum Webhook {
    Create(WebhookCreate),
    List(WebhookList),
    Delete(WebhookDelete),
}

impl Completed for Webhook {
    type Incomplete = incomplete::Webhook;

    fn incomplete(&self) -> Self::Incomplete {
        type Target = incomplete::Webhook;
        match self {
            Self::Create(create) => Target::Create(create.incomplete()),
            Self::Delete(delete) => Target::Delete(delete.incomplete()),
            Self::List(list) => Target::List(list.incomplete()),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebhookCreate {
    pub channel_name: Option<String>,
    pub channel_id: Uuid,
    pub owner: Owner,
}

impl Completed for WebhookCreate {
    type Incomplete = incomplete::WebhookCreate;

    fn incomplete(&self) -> Self::Incomplete {
        incomplete::WebhookCreate {
            channel: self.channel_name.clone(),
            owner: Some(self.owner.name.clone()),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
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

impl Completed for WebhookList {
    type Incomplete = incomplete::WebhookList;

    fn incomplete(&self) -> Self::Incomplete {
        incomplete::WebhookList
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebhookDelete {
    pub webhook_id: String,
}

impl Completed for WebhookDelete {
    type Incomplete = incomplete::WebhookDelete;

    fn incomplete(&self) -> Self::Incomplete {
        incomplete::WebhookDelete {
            id: self.webhook_id.clone(),
        }
    }
}
