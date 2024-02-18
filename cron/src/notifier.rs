use std::{future::Future, sync::Arc, time::Duration};

use tokio::select;
use tokio::time::interval;

use domain::{Event, Infra, TraqClient};

use crate::Notifier;

impl Notifier {
    pub async fn recv_many_with_limit(
        &mut self,
        limit: impl Future<Output = ()> + Send,
    ) -> Vec<Event> {
        let mut res = vec![];
        let recv = async {
            while let Some(e) = self.0.recv().await {
                // TODO: merge
                res.push(e);
            }
        };
        select! {
            _ = recv => {}
            _ = limit => {}
        }
        res
    }

    /// never returns
    #[tracing::instrument(skip_all, fields(period = period.as_millis()))]
    pub async fn run(&mut self, infra: Arc<impl Infra>, period: Duration) {
        let client = infra.traq_client();
        let mut interval = interval(period);
        loop {
            let tick = async {
                interval.tick().await;
                tracing::trace!("tick");
            };
            let events = self.recv_many_with_limit(tick).await;
            for event in events {
                tracing::debug!(event_kind = ?event.kind(), channel_id = ?event.channel_id());
                let res = client
                    .send_message(event.channel_id(), &event.body(), false)
                    .await;
                if let Err(e) = res {
                    tracing::error!("{}", e.into());
                }
            }
        }
    }
}
