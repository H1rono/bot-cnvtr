use std::marker::PhantomData;

use http::HeaderMap;
use serde::{Deserialize, Serialize};

use domain::{Failure, Infra, Webhook};
use usecases::{Bot, WebhookHandler};

#[must_use]
#[derive(Clone)]
pub struct BotWrapper<I: Infra, B: Bot<I>>(pub B, PhantomData<I>);

impl<I: Infra, B: Bot<I>> BotWrapper<I, B> {
    pub fn new(bot: B) -> Self {
        Self(bot, PhantomData)
    }
}

#[must_use]
#[derive(Clone)]
pub struct WHandlerWrapper<I: Infra, W: WebhookHandler<I>>(pub W, PhantomData<I>);

impl<I: Infra, W: WebhookHandler<I>> WHandlerWrapper<I, W> {
    pub fn new(webhook_handler: W) -> Self {
        Self(webhook_handler, PhantomData)
    }
}

impl<I, W> WebhookHandler<I> for WHandlerWrapper<I, W>
where
    I: Infra,
    W: WebhookHandler<I>,
{
    async fn handle(
        &self,
        kind: usecases::WebhookKind,
        infra: &I,
        webhook: Webhook,
        headers: HeaderMap,
        payload: &str,
    ) -> Result<(), Failure> {
        self.0.handle(kind, infra, webhook, headers, payload).await
    }
}

#[must_use]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BotConfig {
    pub verification_token: String,
    #[serde(rename = "bot_name", default = "BotConfig::default_bot_name")]
    pub name: String,
    #[serde(rename = "bot_id")]
    pub id: String,
    #[serde(rename = "bot_user_id")]
    pub user_id: String,
}

impl BotConfig {
    fn default_bot_name() -> String {
        "BOT_cnvtr".to_string()
    }
}
