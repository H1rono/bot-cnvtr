use domain::{Error, Event, EventSubscriber};

use crate::Subscriber;

impl EventSubscriber for Subscriber {
    async fn send(&self, event: Event) -> Result<(), Error> {
        self.0
            .send(event)
            .map_err(|e| anyhow::Error::from(e).into())
    }
}
