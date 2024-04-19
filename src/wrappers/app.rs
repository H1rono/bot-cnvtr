use std::marker::PhantomData;

use http::HeaderMap;
use traq_bot_http::Event;

use domain::{Infra, Webhook};
use usecases::{Bot, WebhookHandler};

#[derive(Clone)]
pub struct BotWrapper<I: Infra, B: Bot<I>>(pub B, PhantomData<I>);

impl<I: Infra, B: Bot<I>> BotWrapper<I, B> {
    pub fn new(bot: B) -> Self {
        Self(bot, PhantomData)
    }
}

impl<I, B> Bot<I> for BotWrapper<I, B>
where
    I: Infra,
    B: Bot<I, Error = I::Error>,
    domain::Error: From<I::Error>,
{
    type Error = domain::Error;

    async fn handle_event(&self, infra: &I, event: Event) -> Result<(), Self::Error> {
        Ok(self.0.handle_event(infra, event).await?)
    }
}

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

    async fn handle(
        &self,
        kind: usecases::WebhookKind,
        infra: &I,
        webhook: Webhook,
        headers: HeaderMap,
        payload: &str,
    ) -> Result<(), Self::Error> {
        Ok(self
            .0
            .handle(kind, infra, webhook, headers, payload)
            .await?)
    }
}
