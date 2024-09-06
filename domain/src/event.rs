use std::borrow::Cow;

use crate::{ChannelId, Event};

impl Event {
    pub fn channel_id(&self) -> &ChannelId {
        &self.channel_id
    }

    #[must_use]
    pub fn kind(&self) -> &str {
        self.kind.as_ref()
    }

    #[must_use]
    pub fn body(&self) -> Cow<'_, str> {
        Cow::Borrowed(self.body.as_ref())
    }

    #[must_use]
    pub fn can_merged(&self, other: &Event) -> bool {
        self.channel_id == other.channel_id && self.kind == other.kind
    }

    pub fn merge(&mut self, other: Event) -> Option<Event> {
        if self.can_merged(&other) {
            self.body.0 += "\n";
            self.body.0 += other.body.as_ref();
            return None;
        }
        Some(other)
    }
}
