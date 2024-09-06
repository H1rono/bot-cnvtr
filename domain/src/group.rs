use crate::newtypes::{GroupId, GroupName};
use crate::{Group, User};

impl Group {
    pub fn new(id: GroupId, name: GroupName, members: &[User]) -> Self {
        Self {
            id,
            name,
            members: members.to_vec(),
        }
    }
}
