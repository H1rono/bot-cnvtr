use clap::Subcommand;

pub mod webhook;

#[derive(Debug, Clone, Subcommand)]
pub enum Sudo {
    #[command(about = "webhookを扱うコマンド")]
    Webhook {
        #[command(subcommand)]
        wh: webhook::Incomplete,
    },
}
