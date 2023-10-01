use thiserror::Error as ThisError;
use traq_client::Error as ClientError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("json parse failed")]
    Serde(#[from] serde_json::Error),
    #[error("sqlx error")]
    Sqlx(#[from] sqlx::Error),
    #[error("traq-client error: {0}")]
    Client(#[from] ClientError),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
