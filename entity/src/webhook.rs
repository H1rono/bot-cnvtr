use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{Channel, Owner};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Webhook {
    pub id: Uuid,
    pub channel: Channel,
    pub owner: Owner,
}

impl Webhook {
    pub fn new(id: Uuid, channel: Channel, owner: Owner) -> Self {
        Self { id, channel, owner }
    }
}
