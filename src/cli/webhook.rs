use clap::{Args, Subcommand};

#[derive(Debug, Clone, Subcommand)]
pub enum Webhook {
    Create(WebhookCreate),
    List(WebhookList),
    Delete(WebhookDelete),
}

#[derive(Debug, Clone, Args)]
pub struct WebhookCreate {
    #[arg(long)]
    channel: Option<String>,
    #[arg(long)]
    owner: Option<String>,
}

#[derive(Debug, Clone, Args)]
pub struct WebhookList;

#[derive(Debug, Clone, Args)]
pub struct WebhookDelete;
