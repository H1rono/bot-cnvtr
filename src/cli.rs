use clap::{Parser, Subcommand};

mod webhook;

#[derive(Debug, Clone, Parser)]
pub struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Commands {
    Webhook {
        #[command(subcommand)]
        wh: webhook::Webhook,
    },
}
