use domain::{Event, EventSubscriber};

#[derive(Clone)]
pub struct EventSubWrapper<S: EventSubscriber + Clone>(pub S);

impl<S: EventSubscriber + Clone> EventSubscriber for EventSubWrapper<S>
where
    domain::Error: From<S::Error>,
{
    type Error = domain::Error;

    async fn send(&self, event: Event) -> Result<(), Self::Error> {
        Ok(self.0.send(event).await?)
    }
}
