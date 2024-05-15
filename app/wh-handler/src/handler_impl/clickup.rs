use http::HeaderMap;
use indoc::formatdoc;
use serde_json::Value;

use domain::{Error, Event, EventSubscriber, Infra, Webhook};

use super::utils::ValueExt;
use crate::WebhookHandlerImpl;

impl WebhookHandlerImpl {
    pub(crate) async fn handle_clickup<I>(
        &self,
        infra: &I,
        webhook: Webhook,
        headers: HeaderMap,
        payload: &str,
    ) -> Result<(), Error>
    where
        I: Infra,
        domain::Error: From<I::Error>,
    {
        let subscriber = infra.event_subscriber();
        let Some(message) = handle(headers, payload)? else {
            return Ok(());
        };
        let kind = "clickup".to_string(); // TODO: event_type
        let event = Event {
            channel_id: webhook.channel_id,
            kind,
            body: message,
        };
        subscriber.send(event).await?;
        Ok(())
    }
}

#[tracing::instrument(target = "wh_handler::gitea::handle", skip_all)]
fn handle(_headers: HeaderMap, payload: &str) -> Result<Option<String>, Error> {
    let payload: Value = serde_json::from_str(payload).map_err(anyhow::Error::from)?;
    let event = payload.get_or_err("event")?.as_str_or_err()?;
    tracing::info!("clickup event: {}", event);
    let message = formatdoc! {
        r#"
            ClickUpからWebhookが送信されました。
            イベント: {event}
            実装は現在工事中です :construction:
        "#,
    };
    Ok(Some(message))
}
