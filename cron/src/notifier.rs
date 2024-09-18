use std::{future::Future, sync::Arc, time::Duration};

use tokio::select;
use tokio::time::interval;

use domain::{Event, Infra, TraqClient};

use crate::Notifier;

impl Notifier {
    async fn recv_many_unstop(&mut self, res: &mut Vec<Event>) {
        while let Some(new_event) = self.0.recv().await {
            let tried_merge = res.iter_mut().try_fold(new_event, |ne, e| e.merge(ne));
            if let Some(e) = tried_merge {
                res.push(e);
            }
        }
    }

    async fn recv_many_with_limit(&mut self, limit: impl Future<Output = ()> + Send) -> Vec<Event> {
        let mut res = Vec::<Event>::new();
        select! {
            () = self.recv_many_unstop(&mut res) => unreachable!(),
            () = limit => {}
        }
        res
    }

    async fn send_events(&self, infra: &impl Infra, events: &[Event]) {
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

    /// never returns
    #[tracing::instrument(skip_all, fields(period = period.as_millis()))]
    pub async fn run(&mut self, infra: Arc<impl Infra>, period: Duration) {
        let mut interval = interval(period);
        loop {
            let tick = async {
                interval.tick().await;
                tracing::trace!("tick");
            };
            let events = self.recv_many_with_limit(tick).await;
            self.send_events(&*infra, &events).await;
        }
    }
}
