use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::Owner;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Webhook {
    pub id: Uuid,
    pub channel_id: Uuid,
    pub owner: Owner,
}

impl Webhook {
    pub fn new(id: Uuid, channel_id: Uuid, owner: Owner) -> Self {
        Self {
            id,
            channel_id,
            owner,
        }
    }
}
