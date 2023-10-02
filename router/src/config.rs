use envy::Result;
use serde::{Deserialize, Serialize};
use traq_bot_http::RequestParser;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub verification_token: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        envy::from_env()
    }
}

impl From<Config> for RequestParser {
    fn from(value: Config) -> Self {
        RequestParser::new(&value.verification_token)
    }
}
