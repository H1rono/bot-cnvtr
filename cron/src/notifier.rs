use std::sync::Arc;
use std::time::Duration;

use tokio::time::interval;
use tokio_stream::{Stream, StreamExt, wrappers::UnboundedReceiverStream};

use domain::{Event, Infra, TraqClient};

use crate::Notifier;

async fn collect_event_stream<S>(mut stream: S) -> Vec<Event>
where
    S: Stream<Item = Event> + Send + Unpin,
{
    let mut events = vec![];
    while let Some(new_event) = stream.next().await {
        let tried_merge = events
            .iter_mut()
            .try_fold(new_event, |ne, e| Event::merge(e, ne));
        let Some(ne) = tried_merge else {
            continue;
        };
        events.push(ne);
    }
    events
}

async fn send_events(infra: &impl Infra, events: &[Event]) {
    if !events.is_empty() {
        tracing::info!("sending {} events...", events.len());
    }
    for event in events {
        tracing::info!(event_kind = event.kind(), channel_id = %event.channel_id());
        let res = infra
            .traq_client()
            .send_message(event.channel_id(), &event.body(), false)
            .await;
        if let Err(e) = res {
            tracing::error!(error = ?e);
        }
    }
}

impl Notifier {
    /// never returns
    #[tracing::instrument(skip_all, fields(period = period.as_millis()))]
    pub async fn run(self, infra: Arc<impl Infra>, period: Duration) {
        let Self(rx) = self;
        let interval = interval(period);
        let mut recv_stream = UnboundedReceiverStream::new(rx).timeout_repeating(interval);
        loop {
            tracing::trace!("tick");
            let event_stream = (&mut recv_stream).map_while(Result::ok);
            let events = collect_event_stream(event_stream).await;
            send_events(&*infra, &events).await;
        }
    }
}
