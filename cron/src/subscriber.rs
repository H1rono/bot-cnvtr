use domain::{Event, EventSubscriber};

use crate::Subscriber;

impl EventSubscriber for Subscriber {
    type Error = anyhow::Error;

    async fn send(&self, event: Event) -> Result<(), Self::Error> {
        self.0.send(event).map_err(anyhow::Error::from)
    }
}
