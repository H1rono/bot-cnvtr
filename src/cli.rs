use clap::{Parser, Subcommand};
use traq_bot_http::payloads::{types::Message, DirectMessageCreatedPayload, MessageCreatedPayload};

pub mod webhook;

#[derive(Debug, Clone, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Commands {
    #[command(about = "webhookを扱うコマンド")]
    Webhook {
        #[command(subcommand)]
        wh: webhook::Incomplete,
    },
}

impl Incomplete<Message> for Commands {
    type Completed = CompletedCmds;

    fn complete(&self, context: Message) -> Self::Completed {
        match self {
            Self::Webhook { wh } => CompletedCmds::Webhook(wh.complete(context)),
        }
    }
}

impl Incomplete<MessageCreatedPayload> for Commands {
    type Completed = CompletedCmds;

    fn complete(&self, context: MessageCreatedPayload) -> Self::Completed {
        self.complete(context.message)
    }
}

impl Incomplete<DirectMessageCreatedPayload> for Commands {
    type Completed = CompletedCmds;

    fn complete(&self, context: DirectMessageCreatedPayload) -> Self::Completed {
        self.complete(context.message)
    }
}

#[derive(Debug, Clone)]
pub enum CompletedCmds {
    Webhook(webhook::Complete),
}

impl Completed for CompletedCmds {
    type Incomplete = Commands;

    fn incomplete(&self) -> Self::Incomplete {
        match self {
            Self::Webhook(wh) => Commands::Webhook {
                wh: wh.incomplete(),
            },
        }
    }
}

pub trait Incomplete<Ctx> {
    type Completed;

    fn complete(&self, context: Ctx) -> Self::Completed;
}

pub trait Completed {
    type Incomplete;

    fn incomplete(&self) -> Self::Incomplete;
}
