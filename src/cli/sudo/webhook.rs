use clap::Subcommand;
use traq_bot_http::payloads::types::Message;
use uuid::Uuid;

use crate::cli;

fn validate(context: &Message) -> bool {
    context.user.name == "H1rono_K"
}

#[derive(Debug, Clone, Subcommand)]
pub enum Incomplete {
    ListAll,
    Delete {
        #[clap(help = "削除するWebhookのID")]
        id: String,
    },
}

impl<'a> cli::Incomplete<&'a Message> for Incomplete {
    type Completed = Completed;

    fn complete(&self, context: &'a Message) -> Self::Completed {
        match self {
            Self::ListAll => Completed::ListAll(ListAll {
                valid: validate(context),
                talking_channel_id: context.channel_id,
            }),
            Self::Delete { id } => Completed::Delete(Delete {
                id: id.clone(),
                valid: validate(context),
                talking_channel_id: context.channel_id,
            }),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Completed {
    ListAll(ListAll),
    Delete(Delete),
}

impl cli::Completed for Completed {
    type Incomplete = Incomplete;

    fn incomplete(&self) -> Self::Incomplete {
        match self {
            Self::ListAll(_) => Incomplete::ListAll,
            Self::Delete(Delete { id, .. }) => Incomplete::Delete { id: id.clone() },
        }
    }
}

#[derive(Debug, Clone)]
pub struct ListAll {
    pub valid: bool,
    pub talking_channel_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct Delete {
    pub id: String,
    pub valid: bool,
    pub talking_channel_id: Uuid,
}
