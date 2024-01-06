use std::future::Future;

use http::HeaderMap;
use serde_json::Value;

use domain::{Infra, Webhook};

pub trait WebhookHandler<I: Infra>: Send + Sync + 'static {
    type Error: Into<domain::Error> + Send + Sync + 'static;

    fn github_webhook(
        &self,
        infra: &I,
        webhook: Webhook,
        headers: HeaderMap,
        payload: Value,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;

    fn gitea_webhook(
        &self,
        infra: &I,
        webhook: Webhook,
        headers: HeaderMap,
        payload: Value,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;

    fn clickup_webhook(
        &self,
        infra: &I,
        webhook: Webhook,
        headers: HeaderMap,
        payload: Value,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;
}
