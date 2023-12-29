use clap::Subcommand;
use traq_bot_http::payloads::types::Message;
use uuid::Uuid;

use domain::{ChannelId, UserId, WebhookId};

fn validate(context: &Message) -> bool {
    context.user.name == "H1rono_K"
}

#[derive(Debug, Clone, Subcommand)]
pub enum Incomplete {
    ListAll,
    Delete {
        #[clap(help = "削除するWebhookのID")]
        id: Uuid,
    },
}

impl<'a> crate::cli::Incomplete<&'a Message> for Incomplete {
    type Completed = Completed;

    fn complete(&self, context: &'a Message) -> Self::Completed {
        match self {
            Self::ListAll => Completed::ListAll(ListAll {
                valid: validate(context),
                talking_channel_id: context.channel_id.into(),
                user_id: context.user.id.into(),
            }),
            Self::Delete { id } => Completed::Delete(Delete {
                id: (*id).into(),
                valid: validate(context),
                talking_channel_id: context.channel_id.into(),
            }),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Completed {
    ListAll(ListAll),
    Delete(Delete),
}

impl crate::cli::Completed for Completed {
    type Incomplete = Incomplete;

    fn incomplete(&self) -> Self::Incomplete {
        match self {
            Self::ListAll(_) => Incomplete::ListAll,
            Self::Delete(Delete { id, .. }) => Incomplete::Delete { id: (*id).into() },
        }
    }
}

#[derive(Debug, Clone)]
pub struct ListAll {
    pub valid: bool,
    pub talking_channel_id: ChannelId,
    pub user_id: UserId,
}

#[derive(Debug, Clone)]
pub struct Delete {
    pub id: WebhookId,
    pub valid: bool,
    pub talking_channel_id: ChannelId,
}
