use crate::id::OwnerId;
use crate::{Group, Owner, OwnerKind, User};

impl Owner {
    pub fn kind(&self) -> OwnerKind {
        match self {
            Self::Group(_) => OwnerKind::Group,
            Self::SigleUser(_) => OwnerKind::SingleUser,
        }
    }

    pub fn id(&self) -> OwnerId {
        match self {
            Self::Group(g) => g.id.0.into(),
            Self::SigleUser(u) => u.id.0.into(),
        }
    }

    #[must_use]
    pub fn name(&self) -> &str {
        match self {
            Self::Group(g) => &g.name,
            Self::SigleUser(u) => &u.name,
        }
    }

    #[must_use]
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
