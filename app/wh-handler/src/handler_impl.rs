use http::HeaderMap;

use domain::{Error, Event, EventSubscriber, Infra, Webhook};
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
        payload: &str,
    ) -> Result<(), Self::Error> {
        let subscriber = infra.event_subscriber();
        let Some(message) = github::handle(headers, payload)? else {
            return Ok(());
        };
        let kind = "github".to_string(); // TODO: event_type
        let event = Event {
            channel_id: webhook.channel_id,
            kind,
            body: message,
        };
        subscriber.send(event).await.map_err(Error::from)?;
        Ok(())
    }

    async fn gitea_webhook(
        &self,
        infra: &I,
        webhook: Webhook,
        headers: HeaderMap,
        payload: &str,
    ) -> Result<(), Self::Error> {
        let subscriber = infra.event_subscriber();
        let Some(message) = gitea::handle(headers, payload)? else {
            return Ok(());
        };
        let kind = "gitea".to_string(); // TODO: event_type
        let event = Event {
            channel_id: webhook.channel_id,
            kind,
            body: message,
        };
        subscriber.send(event).await.map_err(Error::from)?;
        Ok(())
    }

    async fn clickup_webhook(
        &self,
        infra: &I,
        webhook: Webhook,
        headers: HeaderMap,
        payload: &str,
    ) -> Result<(), Self::Error> {
        let subscriber = infra.event_subscriber();
        let Some(message) = clickup::handle(headers, payload)? else {
            return Ok(());
        };
        let kind = "clickup".to_string(); // TODO: event_type
        let event = Event {
            channel_id: webhook.channel_id,
            kind,
            body: message,
        };
        subscriber.send(event).await.map_err(Error::from)?;
        Ok(())
    }
}
