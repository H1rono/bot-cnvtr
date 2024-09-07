use std::fmt;

use crate::newtypes::OwnerId;
use crate::{Group, Owner, OwnerKind, User};

impl Owner {
    pub fn kind(&self) -> OwnerKind {
        match self {
            Self::Group(_) => OwnerKind::Group,
            Self::SingleUser(_) => OwnerKind::SingleUser,
        }
    }

    pub fn id(&self) -> OwnerId {
        match self {
            Self::Group(g) => g.id.0.into(),
            Self::SingleUser(u) => u.id.0.into(),
        }
    }

    #[must_use]
    pub fn name(&self) -> &str {
        match self {
            Self::Group(g) => g.name.as_ref(),
            Self::SingleUser(u) => u.name.as_ref(),
        }
    }

    #[must_use]
    pub fn users(&self) -> Vec<&User> {
        match self {
            Self::Group(g) => g.members.iter().collect(),
            Self::SingleUser(u) => vec![u],
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
        Owner::SingleUser(value)
    }
}
