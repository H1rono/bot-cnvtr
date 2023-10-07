use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("json parse failed")]
    Serde(#[from] serde_json::Error),
    #[error("sqlx error")]
    Sqlx(#[from] sqlx::Error),
    #[error("other")]
    Other(Box<dyn std::error::Error>),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
