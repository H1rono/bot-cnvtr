use crate::id::UserId;
use crate::User;

impl User {
    pub fn new(id: UserId, name: String) -> Self {
        Self { id, name }
    }
}
