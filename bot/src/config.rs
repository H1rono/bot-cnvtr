use envy::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub bot_id: String,
    pub bot_user_id: String,
    pub verification_token: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        envy::from_env()
    }
}
