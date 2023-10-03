use serde::{Deserialize, Serialize};

use crate::{Group, User};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Owner {
    Group(Group),
    SigleUser(User),
}

impl Owner {
    pub fn kind(&self) -> OwnerKind {
        match self {
            Self::Group(_) => OwnerKind::Group,
            Self::SigleUser(_) => OwnerKind::SingleUser,
        }
    }
}

impl From<Group> for Owner {
    fn from(value: Group) -> Self {
        Owner::Group(value)
    }
}

impl From<User> for Owner {
    fn from(value: User) -> Self {
        Owner::SigleUser(value)
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum OwnerKind {
    Group,
    SingleUser,
}
