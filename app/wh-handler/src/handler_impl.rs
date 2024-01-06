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

    async fn github_webhook<'a, H, K, V>(
        &self,
        infra: &I,
        webhook: Webhook,
        headers: H,
        payload: Value,
    ) -> Result<(), Self::Error>
    where
        H: Iterator<Item = (&'a K, &'a V)> + Send + 'a,
        K: AsRef<[u8]> + Send + ?Sized + 'static,
        V: AsRef<[u8]> + Send + ?Sized + 'static,
    {
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

    async fn gitea_webhook<'a, H, K, V>(
        &self,
        infra: &I,
        webhook: Webhook,
        headers: H,
        payload: Value,
    ) -> Result<(), Self::Error>
    where
        H: Iterator<Item = (&'a K, &'a V)> + Send + 'a,
        K: AsRef<[u8]> + Send + ?Sized + 'static,
        V: AsRef<[u8]> + Send + ?Sized + 'static,
    {
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

    async fn clickup_webhook<'a, H, K, V>(
        &self,
        infra: &I,
        webhook: Webhook,
        headers: H,
        payload: Value,
    ) -> Result<(), Self::Error>
    where
        H: Iterator<Item = (&'a K, &'a V)> + Send + 'a,
        K: AsRef<[u8]> + Send + ?Sized + 'static,
        V: AsRef<[u8]> + Send + ?Sized + 'static,
    {
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
