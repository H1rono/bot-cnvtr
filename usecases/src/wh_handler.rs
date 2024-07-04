use std::future::Future;

use http::HeaderMap;

use domain::{Infra, Webhook};

#[must_use]
#[derive(Debug, Clone, Copy)]
pub enum WebhookKind {
    GitHub,
    Gitea,
    Clickup,
}

pub trait WebhookHandler<I: Infra>: Send + Sync + 'static {
    type Error: Into<domain::Error> + Send + Sync + 'static;

    fn handle(
        &self,
        kind: WebhookKind,
        infra: &I,
        webhook: Webhook,
        headers: HeaderMap,
        payload: &str,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;
}
