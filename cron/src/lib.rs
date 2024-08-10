pub mod notifier;
pub mod subscriber;

use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

use domain::Event;

#[must_use]
#[derive(Debug, Clone)]
pub struct Subscriber(pub(crate) UnboundedSender<Event>);

#[must_use]
#[derive(Debug)]
pub struct Notifier(pub(crate) UnboundedReceiver<Event>);

pub fn channel() -> (Subscriber, Notifier) {
    let (tx, rx) = mpsc::unbounded_channel();
    (Subscriber(tx), Notifier(rx))
}
