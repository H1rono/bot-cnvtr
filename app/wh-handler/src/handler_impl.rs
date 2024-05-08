use http::HeaderMap;

use domain::{Infra, Webhook};
use usecases::{WebhookHandler, WebhookKind};

use crate::WebhookHandlerImpl;

mod clickup;
mod gitea;
mod github;
mod utils;

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

impl<I> WebhookHandler<I> for WebhookHandlerImpl
where
    I: Infra,
    domain::Error: From<I::Error>,
{
    type Error = domain::Error;

    async fn handle(
        &self,
        kind: WebhookKind,
        infra: &I,
        webhook: Webhook,
        headers: HeaderMap,
        payload: &str,
    ) -> Result<(), Self::Error> {
        match kind {
            WebhookKind::Clickup => {
                self.handle_clickup(infra, webhook, headers, payload)
                    .await?
            }
            WebhookKind::GitHub => self.handle_github(infra, webhook, headers, payload).await?,
            WebhookKind::Gitea => self.handle_gitea(infra, webhook, headers, payload).await?,
        };
        Ok(())
    }
}
