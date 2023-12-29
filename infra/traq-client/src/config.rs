use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub bot_access_token: String,
}

impl Config {
    pub fn new(access_token: &str) -> Self {
        Self {
            bot_access_token: access_token.to_string(),
        }
    }

    pub fn from_env() -> envy::Result<Self> {
        envy::from_env()
    }
}
