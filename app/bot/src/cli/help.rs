use domain::{ChannelId, UserId};

#[derive(Debug, Clone, Copy)]
pub enum CompleteHelp {
    Channel(ChannelId),
    Dm(UserId),
}
