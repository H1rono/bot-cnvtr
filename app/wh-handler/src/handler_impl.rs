use http::HeaderMap;
use serde_json::Value;

use domain::{Error, Infra, TraqClient, Webhook};
use usecases::WebhookHandler;

mod clickup;
mod gitea;
mod github;
mod utils;

#[derive(Debug, Clone)]
pub struct WebhookHandlerImpl;

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

impl<I: Infra> WebhookHandler<I> for WebhookHandlerImpl
where
    Error: From<I::Error>,
{
    type Error = Error;

    async fn github_webhook(
        &self,
        infra: &I,
        webhook: Webhook,
        headers: HeaderMap,
        payload: Value,
    ) -> Result<(), Self::Error> {
        let client = infra.traq_client();
        let Some(message) = github::handle(headers, payload)? else {
            return Ok(());
        };
        client
            .send_message(&webhook.channel_id, message.trim(), false)
            .await
            .map_err(Error::from)?;
        Ok(())
    }

    async fn gitea_webhook(
        &self,
        infra: &I,
        webhook: Webhook,
        headers: HeaderMap,
        payload: Value,
    ) -> Result<(), Self::Error> {
        let client = infra.traq_client();
        let Some(message) = gitea::handle(headers, payload)? else {
            return Ok(());
        };
        client
            .send_message(&webhook.channel_id, message.trim(), false)
            .await
            .map_err(Error::from)?;
        Ok(())
    }

    async fn clickup_webhook(
        &self,
        infra: &I,
        webhook: Webhook,
        headers: HeaderMap,
        payload: Value,
    ) -> Result<(), Self::Error> {
        let client = infra.traq_client();
        let Some(message) = clickup::handle(headers, payload)? else {
            return Ok(());
        };
        client
            .send_message(&webhook.channel_id, message.trim(), false)
            .await
            .map_err(Error::from)?;
        Ok(())
    }
}
