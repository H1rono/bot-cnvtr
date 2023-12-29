use serde_json::Value;

use usecases::WebhookHandler;

use crate::{Error, Result};

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
    type Error = Error;

    fn github_webhook<'a, H, K, V>(&self, headers: H, payload: Value) -> Result<Option<String>>
    where
        H: Iterator<Item = (&'a K, &'a V)>,
        K: AsRef<[u8]> + ?Sized + 'static,
        V: AsRef<[u8]> + ?Sized + 'static,
    {
        github::handle(headers, payload)
    }

    fn gitea_webhook<'a, H, K, V>(&self, headers: H, payload: Value) -> Result<Option<String>>
    where
        H: Iterator<Item = (&'a K, &'a V)>,
        K: AsRef<[u8]> + ?Sized + 'static,
        V: AsRef<[u8]> + ?Sized + 'static,
    {
        gitea::handle(headers, payload)
    }

    fn clickup_webhook<'a, H, K, V>(&self, headers: H, payload: Value) -> Result<Option<String>>
    where
        H: Iterator<Item = (&'a K, &'a V)>,
        K: AsRef<[u8]> + ?Sized + 'static,
        V: AsRef<[u8]> + ?Sized + 'static,
    {
        clickup::handle(headers, payload)
    }
}
