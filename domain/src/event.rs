use std::{borrow::Cow, future::Future};

use crate::ChannelId;

#[derive(Clone, Debug)]
pub struct Event {
    pub channel_id: ChannelId,
    pub kind: String,
    pub body: String,
}

pub trait EventSubscriber: Clone + Send + Sync + 'static {
    type Error: Into<crate::Error> + Send + Sync + 'static;

    fn send(&self, event: Event) -> impl Future<Output = Result<(), Self::Error>> + Send;
}

impl Event {
    pub fn channel_id(&self) -> &ChannelId {
        &self.channel_id
    }

    pub fn kind(&self) -> &str {
        &self.kind
    }

    pub fn body(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.body)
    }

    pub fn can_merged(&self, other: &Event) -> bool {
        self.channel_id == other.channel_id && self.kind == other.kind
    }

    pub fn merge(&mut self, other: Event) -> Option<Event> {
        if self.can_merged(&other) {
            self.kind += "\n";
            self.kind += &other.kind;
            return None;
        }
        Some(other)
    }
}
