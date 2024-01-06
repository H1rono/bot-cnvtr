use std::future::Future;

use serde_json::Value;

use domain::{Infra, Webhook};

pub trait WebhookHandler<I: Infra>: Send + Sync + 'static {
    type Error: Into<domain::Error> + Send + Sync + 'static;

    fn github_webhook<'a, H, K, V>(
        &self,
        infra: &I,
        webhook: Webhook,
        headers: H,
        payload: Value,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send
    where
        H: Iterator<Item = (&'a K, &'a V)> + Send + 'a,
        K: AsRef<[u8]> + Send + ?Sized + 'static,
        V: AsRef<[u8]> + Send + ?Sized + 'static;

    fn gitea_webhook<'a, H, K, V>(
        &self,
        infra: &I,
        webhook: Webhook,
        headers: H,
        payload: Value,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send
    where
        H: Iterator<Item = (&'a K, &'a V)> + Send + 'a,
        K: AsRef<[u8]> + Send + ?Sized + 'static,
        V: AsRef<[u8]> + Send + ?Sized + 'static;

    fn clickup_webhook<'a, H, K, V>(
        &self,
        infra: &I,
        webhook: Webhook,
        headers: H,
        payload: Value,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send
    where
        H: Iterator<Item = (&'a K, &'a V)> + Send + 'a,
        K: AsRef<[u8]> + Send + ?Sized + 'static,
        V: AsRef<[u8]> + Send + ?Sized + 'static;
}
