use std::marker::PhantomData;

use http::HeaderMap;

use domain::{Infra, Webhook};
use usecases::WebhookHandler;

#[derive(Clone)]
pub struct WHandlerWrapper<I: Infra, W: WebhookHandler<I>>(pub W, PhantomData<I>);

impl<I: Infra, W: WebhookHandler<I>> WHandlerWrapper<I, W> {
    pub fn new(webhook_handler: W) -> Self {
        Self(webhook_handler, PhantomData)
    }
}

impl<I, W> WebhookHandler<I> for WHandlerWrapper<I, W>
where
    I: Infra<Error = domain::Error>,
    W: WebhookHandler<I>,
    domain::Error: From<W::Error>,
{
    type Error = domain::Error;

    async fn github_webhook(
        &self,
        infra: &I,
        webhook: Webhook,
        headers: HeaderMap,
        payload: &str,
    ) -> Result<(), Self::Error> {
        Ok(self
            .0
            .github_webhook(infra, webhook, headers, payload)
            .await?)
    }

    async fn gitea_webhook(
        &self,
        infra: &I,
        webhook: Webhook,
        headers: HeaderMap,
        payload: &str,
    ) -> Result<(), Self::Error> {
        Ok(self
            .0
            .gitea_webhook(infra, webhook, headers, payload)
            .await?)
    }

    async fn clickup_webhook(
        &self,
        infra: &I,
        webhook: Webhook,
        headers: HeaderMap,
        payload: &str,
    ) -> Result<(), Self::Error> {
        Ok(self
            .0
            .clickup_webhook(infra, webhook, headers, payload)
            .await?)
    }
}
