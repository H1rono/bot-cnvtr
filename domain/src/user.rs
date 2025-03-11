use crate::User;
use crate::newtypes::{UserId, UserName};

impl User {
    pub fn new(id: UserId, name: UserName) -> Self {
        Self { id, name }
    }
}
