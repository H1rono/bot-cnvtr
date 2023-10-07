use hyper::http::HeaderMap;
use serde_json::Value;

use crate::{Result, WebhookHandler};

mod clickup;
mod gitea;
mod github;
mod utils;

#[derive(Debug, Clone)]
pub struct WebhookHandlerImpl;

impl WebhookHandlerImpl {
    pub fn new() -> Self {
        Self
    }
}

impl Default for WebhookHandlerImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl WebhookHandler for WebhookHandlerImpl {
    fn github_webhook(&self, headers: HeaderMap, payload: Value) -> Result<Option<String>> {
        github::handle(headers, payload)
    }

    fn gitea_webhook(&self, headers: HeaderMap, payload: Value) -> Result<Option<String>> {
        gitea::handle(headers, payload)
    }

    fn clickup_webhook(&self, headers: HeaderMap, payload: Value) -> Result<Option<String>> {
        clickup::handle(headers, payload)
    }
}
