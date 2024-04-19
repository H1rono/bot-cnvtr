use serde::{Deserialize, Serialize};

use crate::Owner;

crate::macros::newtype_id! {Webhook}
crate::macros::newtype_id! {Channel}

#[must_use]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Webhook {
    pub id: WebhookId,
    pub channel_id: ChannelId,
    pub owner: Owner,
}

impl Webhook {
    pub fn new(id: WebhookId, channel_id: ChannelId, owner: Owner) -> Self {
        Self {
            id,
            channel_id,
            owner,
        }
    }
}
