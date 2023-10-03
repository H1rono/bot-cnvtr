use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Channel {
    pub id: Uuid,
    pub name: String,
}

impl Channel {
    pub fn new(id: Uuid, name: String) -> Self {
        Self { id, name }
    }
}
