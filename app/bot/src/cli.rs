use clap::{Parser, Subcommand};
use traq_bot_http::payloads::{types::Message, DirectMessageCreatedPayload, MessageCreatedPayload};

pub mod help;
pub mod sudo;
pub mod webhook;

#[must_use]
#[derive(Debug, Clone, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Commands,
}

#[must_use]
#[derive(Debug, Clone, Subcommand)]
pub enum Commands {
    #[command(about = "webhookを扱うコマンド")]
    Webhook {
        #[command(subcommand)]
        wh: webhook::Incomplete,
    },
    #[command(about = "sudoコマンド")]
    Sudo {
        #[command(subcommand)]
        sudo: sudo::Sudo,
    },
    #[command(about = "cat help.md")]
    PrintHelp,
}

impl<'a> Incomplete<&'a MessageCreatedPayload> for Commands {
    type Completed = CompletedCmds;

    fn complete(&self, context: &'a MessageCreatedPayload) -> Self::Completed {
        match self {
            Self::Webhook { wh } => CompletedCmds::Webhook(wh.complete(context)),
            Self::Sudo { sudo } => CompletedCmds::Sudo(sudo.complete(context)),
            Self::PrintHelp => CompletedCmds::PrintHelp(help::CompleteHelp::Channel(
                context.message.channel_id.into(),
            )),
        }
    }
}

impl<'a> Incomplete<&'a DirectMessageCreatedPayload> for Commands {
    type Completed = CompletedCmds;

    fn complete(&self, context: &'a DirectMessageCreatedPayload) -> Self::Completed {
        match self {
            Self::Webhook { wh } => CompletedCmds::Webhook(wh.complete(context)),
            Self::Sudo { sudo } => CompletedCmds::Sudo(sudo.complete(context)),
            Self::PrintHelp => {
                CompletedCmds::PrintHelp(help::CompleteHelp::Dm(context.message.user.id.into()))
            }
        }
    }
}

#[must_use]
#[derive(Debug, Clone)]
pub enum CompletedCmds {
    Webhook(webhook::Complete),
    Sudo(sudo::SudoCompleted),
    PrintHelp(help::CompleteHelp),
}

impl Completed for CompletedCmds {
    type Incomplete = Commands;

    fn incomplete(&self) -> Self::Incomplete {
        match self {
            Self::Webhook(wh) => Commands::Webhook {
                wh: wh.incomplete(),
            },
            Self::Sudo(sudo) => Commands::Sudo {
                sudo: sudo.incomplete(),
            },
            Self::PrintHelp(_) => Commands::PrintHelp,
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
