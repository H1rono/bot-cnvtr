use crate::newtypes::{ChannelId, WebhookId};
use crate::{Owner, Webhook};

impl Webhook {
    pub fn new(id: WebhookId, channel_id: ChannelId, owner: Owner) -> Self {
        Self {
            id,
            channel_id,
            owner,
        }
    }
}
