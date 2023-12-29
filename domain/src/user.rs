use serde::{Deserialize, Serialize};


crate::macros::newtype_id! {User}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub id: UserId,
    pub name: String,
}

impl User {
    pub fn new(id: UserId, name: String) -> Self {
        Self {
            id,
            name,
        }
    }
}
