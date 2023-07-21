use clap::{Parser, Subcommand};
use traq_bot_http::payloads::{types::Message, DirectMessageCreatedPayload, MessageCreatedPayload};

pub mod sudo;
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

impl<'a> Incomplete<&'a MessageCreatedPayload> for Commands {
    type Completed = CompletedCmds;

    fn complete(&self, context: &'a MessageCreatedPayload) -> Self::Completed {
        match self {
            Self::Webhook { wh } => CompletedCmds::Webhook(wh.complete(context)),
        }
    }
}

impl<'a> Incomplete<&'a DirectMessageCreatedPayload> for Commands {
    type Completed = CompletedCmds;

    fn complete(&self, context: &'a DirectMessageCreatedPayload) -> Self::Completed {
        match self {
            Self::Webhook { wh } => CompletedCmds::Webhook(wh.complete(context)),
        }
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

impl<'a, T> Incomplete<&'a MessageCreatedPayload> for T
where
    T: Incomplete<&'a Message>,
{
    type Completed = T::Completed;

    fn complete(&self, context: &'a MessageCreatedPayload) -> Self::Completed {
        self.complete(&context.message)
    }
}

impl<'a, T> Incomplete<&'a DirectMessageCreatedPayload> for T
where
    T: Incomplete<&'a Message>,
{
    type Completed = T::Completed;

    fn complete(&self, context: &'a DirectMessageCreatedPayload) -> Self::Completed {
        self.complete(&context.message)
    }
}
