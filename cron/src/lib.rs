pub mod notifier;
pub mod subscriber;

use tokio::sync::mpsc::{self, Receiver, Sender};

use domain::Event;

#[derive(Debug, Clone)]
pub struct Subscriber(pub(crate) Sender<Event>);

#[derive(Debug)]
pub struct Notifier(pub(crate) Receiver<Event>);

pub fn channel(buffer: usize) -> (Subscriber, Notifier) {
    let (tx, rx) = mpsc::channel(buffer);
    (Subscriber(tx), Notifier(rx))
}
