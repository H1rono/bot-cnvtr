use http::HeaderMap;
use indoc::formatdoc;
use serde_json::Value;

use domain::{Event, EventSubscriber, Failure, Infra, Webhook};

use super::utils::ValueExt;
use crate::WebhookHandlerImpl;

impl WebhookHandlerImpl {
    pub(crate) async fn handle_clickup<I>(
        &self,
        infra: &I,
        webhook: Webhook,
        headers: HeaderMap,
        payload: &str,
    ) -> Result<(), Failure>
    where
        I: Infra,
    {
        let subscriber = infra.event_subscriber();
        let Some(message) = handle(headers, payload)? else {
            return Ok(());
        };
        let kind = "clickup".to_string().into(); // TODO: event_type
        let event = Event {
            channel_id: webhook.channel_id,
            kind,
            body: message.into(),
        };
        subscriber.send(event).await?;
        Ok(())
    }
}

#[tracing::instrument(target = "wh_handler::gitea::handle", skip_all)]
fn handle(_headers: HeaderMap, payload: &str) -> Result<Option<String>, Failure> {
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
