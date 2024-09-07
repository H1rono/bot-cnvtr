use std::fmt;
use std::iter::FusedIterator;
use std::ops::Range;

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

    pub fn iter_users(&self) -> IterUsers<'_> {
        IterUsers::new(self)
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

#[must_use]
#[derive(Clone)]
pub struct IterUsers<'a> {
    inner: &'a Owner,
    rng: Range<usize>,
}

impl<'a> IterUsers<'a> {
    pub(crate) fn new(inner: &'a Owner) -> Self {
        let rng = match inner {
            Owner::Group(g) => 0..g.members.len(),
            Owner::SingleUser(_) => 0..1,
        };
        Self { inner, rng }
    }

    fn get(&self, i: usize) -> Option<&'a User> {
        match self.inner {
            Owner::Group(g) => g.members.get(i),
            Owner::SingleUser(u) if i == 0 => Some(u),
            Owner::SingleUser(_) => None,
        }
    }
}

impl<'a> fmt::Debug for IterUsers<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("IterUsers").field(self.inner).finish()
    }
}

impl<'a> Iterator for IterUsers<'a> {
    type Item = &'a User;

    fn next(&mut self) -> Option<Self::Item> {
        self.rng.next().and_then(|i| self.get(i))
    }
}

impl<'a> ExactSizeIterator for IterUsers<'a> {
    fn len(&self) -> usize {
        self.rng.len()
    }
}

impl<'a> DoubleEndedIterator for IterUsers<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.rng.next_back().and_then(|i| self.get(i))
    }
}

impl<'a> FusedIterator for IterUsers<'a> {}
