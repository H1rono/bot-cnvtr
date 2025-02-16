use std::fmt;

#[must_use]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, thiserror::Error)]
pub enum RejectKind {
    #[error("not found")]
    NotFound,
    #[error("bad request")]
    BadRequest,
    #[error("unauthorized")]
    Unauthorized,
    #[error("not implemented")]
    NotImplemented,
    #[error("permission denied")]
    PermissionDenied,
}

#[must_use]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Reject {
    kind: RejectKind,
    message: String,
}

impl fmt::Display for Reject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { kind, message } = self;
        write!(f, "{kind}: {message}")
    }
}

impl Reject {
    pub fn new(kind: RejectKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }

    pub fn kind(&self) -> RejectKind {
        self.kind
    }

    #[must_use]
    pub fn as_message(&self) -> &str {
        &self.message
    }

    #[must_use]
    pub fn into_message(self) -> String {
        self.message
    }
}

#[must_use]
pub enum Failure {
    Reject(Reject),
    Error(anyhow::Error),
}

impl fmt::Debug for Failure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Reject(r) => fmt::Debug::fmt(r, f),
            Self::Error(e) => fmt::Debug::fmt(e, f),
        }
    }
}

impl fmt::Display for Failure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Reject(r) => fmt::Display::fmt(r, f),
            Self::Error(e) => fmt::Debug::fmt(e, f),
        }
    }
}

impl From<Reject> for Failure {
    fn from(value: Reject) -> Self {
        Self::Reject(value)
    }
}

impl From<anyhow::Error> for Failure {
    fn from(value: anyhow::Error) -> Self {
        Self::Error(value)
    }
}

impl Failure {
    pub fn reject_not_found(message: impl Into<String>) -> Self {
        Reject::new(RejectKind::NotFound, message).into()
    }

    pub fn reject_bad_request(message: impl Into<String>) -> Self {
        Reject::new(RejectKind::BadRequest, message).into()
    }

    pub fn reject_unauthorized(message: impl Into<String>) -> Self {
        Reject::new(RejectKind::Unauthorized, message).into()
    }

    pub fn reject_not_implemented(message: impl Into<String>) -> Self {
        Reject::new(RejectKind::NotImplemented, message).into()
    }

    pub fn reject_permission_denied(message: impl Into<String>) -> Self {
        Reject::new(RejectKind::PermissionDenied, message).into()
    }
}
