use serde_json::Value;

mod error;
mod handler_impl;

pub use error::{Error, Result};
pub use handler_impl::WebhookHandlerImpl;

pub trait WebhookHandler: Clone + Send + Sync + 'static {
    fn github_webhook<'a, H, K, V>(&self, headers: H, payload: Value) -> Result<Option<String>>
    where
        H: Iterator<Item = (&'a K, &'a V)>,
        K: AsRef<[u8]> + ?Sized + 'static,
        V: AsRef<[u8]> + ?Sized + 'static;

    fn gitea_webhook<'a, H, K, V>(&self, headers: H, payload: Value) -> Result<Option<String>>
    where
        H: Iterator<Item = (&'a K, &'a V)>,
        K: AsRef<[u8]> + ?Sized + 'static,
        V: AsRef<[u8]> + ?Sized + 'static;

    fn clickup_webhook<'a, H, K, V>(&self, headers: H, payload: Value) -> Result<Option<String>>
    where
        H: Iterator<Item = (&'a K, &'a V)>,
        K: AsRef<[u8]> + ?Sized + 'static,
        V: AsRef<[u8]> + ?Sized + 'static;
}
