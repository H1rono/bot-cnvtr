use traq::apis::Error as ApiError;

use super::Error;

impl<T> From<ApiError<T>> for Error {
    fn from(value: ApiError<T>) -> Self {
        use ApiError::*;
        match value {
            Reqwest(e) => e.into(),
            Serde(e) => e.into(),
            Io(e) => e.into(),
            ResponseError(e) => Self::BadResponse {
                status: e.status,
                content: e.content,
            },
        }
    }
}
