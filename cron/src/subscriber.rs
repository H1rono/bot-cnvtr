use anyhow::Context;

use domain::{Event, EventSubscriber, Failure};

use crate::Subscriber;

impl EventSubscriber for Subscriber {
    async fn send(&self, event: Event) -> Result<(), Failure> {
        self.0.send(event).context("Failed to send event")?;
        Ok(())
    }
}
