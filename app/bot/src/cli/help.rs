use domain::{ChannelId, UserId};

// FIXME
#[allow(clippy::module_name_repetitions)]
#[must_use]
#[derive(Debug, Clone, Copy)]
pub enum CompleteHelp {
    Channel(ChannelId),
    Dm(UserId),
}
