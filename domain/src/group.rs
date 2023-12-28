use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::User;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Group {
    pub id: Uuid,
    pub name: String,
    pub members: Vec<User>,
}

impl Group {
    pub fn new(id: Uuid, name: String, members: &[User]) -> Self {
        Self {
            id,
            name,
            members: members.to_vec(),
        }
    }
}
