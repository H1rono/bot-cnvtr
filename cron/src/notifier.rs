use std::{sync::Arc, time::Duration};

use tokio::time::interval;
use tokio_stream::{wrappers::UnboundedReceiverStream, StreamExt};

use domain::{Event, Infra, TraqClient};

use crate::Notifier;

async fn send_events(infra: &impl Infra, events: &[Event]) {
    if !events.is_empty() {
        tracing::info!("sending {} events...", events.len());
    }
    for event in events {
        tracing::debug!(event_kind = ?event.kind(), channel_id = ?event.channel_id());
        let res = infra
            .traq_client()
            .send_message(event.channel_id(), &event.body(), false)
            .await;
        if let Err(e) = res {
            tracing::error!("{}", e.into());
        }
    }
}

impl Notifier {
    /// never returns
    #[tracing::instrument(skip_all, fields(period = period.as_millis()))]
    pub async fn run(self, infra: Arc<impl Infra>, period: Duration) {
        let interval = interval(period);
        let mut recv_stream = UnboundedReceiverStream::new(self.0).timeout_repeating(interval);
        loop {
            tracing::trace!("tick");
            let events = (&mut recv_stream)
                .take_while(Result::is_ok)
                .collect::<Result<Vec<_>, _>>()
                .await
                .unwrap();
            send_events(&*infra, &events).await;
        }
    }
}
