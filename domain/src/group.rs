use crate::id::GroupId;
use crate::{Group, User};

impl Group {
    pub fn new(id: GroupId, name: String, members: &[User]) -> Self {
        Self {
            id,
            name,
            members: members.to_vec(),
        }
    }
}
