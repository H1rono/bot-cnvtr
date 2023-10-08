use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

    pub fn id(&self) -> Uuid {
        match self {
            Self::Group(g) => g.id,
            Self::SigleUser(u) => u.id,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Group(g) => &g.name,
            Self::SigleUser(u) => &u.name,
        }
    }

    pub fn users(&self) -> Vec<&User> {
        match self {
            Self::Group(g) => g.members.iter().collect(),
            Self::SigleUser(u) => vec![u],
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum OwnerKind {
    Group,
    SingleUser,
}
