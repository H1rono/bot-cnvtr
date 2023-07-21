use clap::Subcommand;
use traq_bot_http::payloads::types::Message;

use super::{Completed, Incomplete};

pub mod webhook;

#[derive(Debug, Clone, Subcommand)]
pub enum Sudo {
    #[command(about = "webhookを扱うコマンド")]
    Webhook {
        #[command(subcommand)]
        wh: webhook::Incomplete,
    },
}

impl<'a> Incomplete<&'a Message> for Sudo {
    type Completed = SudoCompleted;

    fn complete(&self, context: &'a Message) -> Self::Completed {
        match self {
            Self::Webhook { wh } => SudoCompleted::Webhook(wh.complete(context)),
        }
    }
}

#[derive(Debug, Clone)]
pub enum SudoCompleted {
    Webhook(webhook::Completed),
}

impl Completed for SudoCompleted {
    type Incomplete = Sudo;

    fn incomplete(&self) -> Self::Incomplete {
        match self {
            Self::Webhook(wh) => Sudo::Webhook {
                wh: wh.incomplete(),
            },
        }
    }
}
