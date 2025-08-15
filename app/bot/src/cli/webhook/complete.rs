use serde::{Deserialize, Serialize};

use domain::{ChannelId, OwnerId, OwnerKind, User, WebhookId};

#[must_use]
#[derive(Debug, Clone)]
pub enum Webhook {
    Create(WebhookCreate),
    List(WebhookList),
    Delete(WebhookDelete),
}

#[must_use]
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

#[must_use]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebhookList {
    pub user: User,
}

#[must_use]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebhookDelete {
    pub user: User,
    pub talking_channel_id: ChannelId,
    pub webhook_id: WebhookId,
}
