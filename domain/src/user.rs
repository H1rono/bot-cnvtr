use crate::newtypes::{UserId, UserName};
use crate::User;

impl User {
    pub fn new(id: UserId, name: UserName) -> Self {
        Self { id, name }
    }
}
