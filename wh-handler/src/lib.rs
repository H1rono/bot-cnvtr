use hyper::http::HeaderMap;
use serde_json::Value;

mod error;
mod handler_impl;

pub use error::{Error, Result};
pub use handler_impl::WebhookHandlerImpl;

pub trait WebhookHandler: Clone + Send + Sync + 'static {
    fn github_webhook(&self, headers: HeaderMap, payload: Value) -> Result<Option<String>>;

    fn gitea_webhook(&self, headers: HeaderMap, payload: Value) -> Result<Option<String>>;

    fn clickup_webhook(&self, headers: HeaderMap, payload: Value) -> Result<Option<String>>;
}
