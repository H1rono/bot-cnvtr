use clap::{Parser, Subcommand};

pub mod webhook;

#[derive(Debug, Clone, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Commands {
    Webhook {
        #[command(subcommand)]
        wh: webhook::Webhook,
    },
}
