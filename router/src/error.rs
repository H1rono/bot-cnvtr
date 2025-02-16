use std::fmt;

use axum::response::IntoResponse;

use domain::Failure;

#[must_use]
pub struct Error(pub Failure);

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl std::error::Error for Error {}

impl From<Failure> for Error {
    fn from(value: Failure) -> Self {
        Self(value)
    }
}

impl From<anyhow::Error> for Error {
    fn from(value: anyhow::Error) -> Self {
        Failure::from(value).into()
    }
}

impl IntoResponse for Error {
    #[tracing::instrument(skip_all, target = "router::error::Error::into_response")]
    fn into_response(self) -> axum::response::Response {
        use http::StatusCode;

        use domain::error::RejectKind;
        use Failure::Reject;

        match self.0 {
            Reject(r) if r.kind() == RejectKind::BadRequest => {
                let message = r.into_message();
                (StatusCode::BAD_REQUEST, message).into_response()
            }
            Reject(r) if r.kind() == RejectKind::NotFound => {
                let message = r.into_message();
                (StatusCode::NOT_FOUND, message).into_response()
            }
            Reject(r) if r.kind() == RejectKind::NotImplemented => {
                let message = r.into_message();
                (StatusCode::NOT_IMPLEMENTED, message).into_response()
            }
            Reject(r) if r.kind() == RejectKind::Unauthorized => {
                let message = r.into_message();
                (StatusCode::UNAUTHORIZED, message).into_response()
            }
            Reject(r) => {
                let message = r.to_string();
                (StatusCode::BAD_REQUEST, message).into_response()
            }
            Failure::Error(e) => {
                tracing::error!("{e:?}");
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
