use http::HeaderMap;

use domain::{Failure, Infra, Webhook};

#[must_use]
#[derive(Debug, Clone, Copy)]
pub enum WebhookKind {
    GitHub,
    Gitea,
    Clickup,
}

#[must_use]
pub trait WebhookHandler<I: Infra>: Send + Sync + 'static {
    fn handle(
        &self,
        kind: WebhookKind,
        infra: &I,
        webhook: Webhook,
        headers: HeaderMap,
        payload: &str,
    ) -> impl Future<Output = Result<(), Failure>> + Send;
}
