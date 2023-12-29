use serde::{Deserialize, Serialize};

use crate::User;

crate::macros::newtype_id! {Group}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Group {
    pub id: GroupId,
    pub name: String,
    pub members: Vec<User>,
}

impl Group {
    pub fn new(id: GroupId, name: String, members: &[User]) -> Self {
        Self {
            id,
            name,
            members: members.to_vec(),
        }
    }
}
