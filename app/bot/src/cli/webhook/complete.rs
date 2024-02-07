use serde::{Deserialize, Serialize};

use domain::{ChannelId, OwnerId, OwnerKind, User, WebhookId};

use super::incomplete;
use crate::cli::Completed;

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
    pub user: User,
    pub in_dm: bool,
    pub talking_channel_id: ChannelId,
    pub channel_name: Option<String>,
    pub channel_id: ChannelId,
    pub channel_dm: bool,
    pub owner_id: OwnerId,
    pub owner_name: String,
    pub owner_kind: OwnerKind,
}

impl Completed for WebhookCreate {
    type Incomplete = incomplete::WebhookCreate;

    fn incomplete(&self) -> Self::Incomplete {
        incomplete::WebhookCreate {
            channel: self.channel_name.clone(),
            owner: Some(self.owner_name.clone()),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebhookList {
    pub user: User,
}

impl Completed for WebhookList {
    type Incomplete = incomplete::WebhookList;

    fn incomplete(&self) -> Self::Incomplete {
        incomplete::WebhookList
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebhookDelete {
    pub user: User,
    pub talking_channel_id: ChannelId,
    pub webhook_id: WebhookId,
}

impl Completed for WebhookDelete {
    type Incomplete = incomplete::WebhookDelete;

    fn incomplete(&self) -> Self::Incomplete {
        incomplete::WebhookDelete {
            id: self.webhook_id.into(),
        }
    }
}
