use clap::{Parser, Subcommand};

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

pub trait Incomplete<Ctx> {
    type Completed;

    fn complete(&self, context: Ctx) -> Self::Completed;
}

pub trait Completed {
    type Incomplete;

    fn incomplete(&self) -> Self::Incomplete;
}
