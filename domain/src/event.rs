use std::borrow::Cow;

use crate::{ChannelId, Event};

impl Event {
    pub fn channel_id(&self) -> &ChannelId {
        &self.channel_id
    }

    #[must_use]
    pub fn kind(&self) -> &str {
        &self.kind
    }

    #[must_use]
    pub fn body(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.body)
    }

    #[must_use]
    pub fn can_merged(&self, other: &Event) -> bool {
        self.channel_id == other.channel_id && self.kind == other.kind
    }

    pub fn merge(&mut self, other: Event) -> Option<Event> {
        if self.can_merged(&other) {
            self.body += "\n";
            self.body += &other.body;
            return None;
        }
        Some(other)
    }
}
