use clap::{Args, Subcommand};

#[derive(Debug, Clone, Subcommand)]
pub enum Webhook {
    Create(WebhookCreate),
    List(WebhookList),
    Delete(WebhookDelete),
}

#[derive(Debug, Clone, Args)]
pub struct WebhookCreate {
    #[arg(
        short,
        long,
        help = "webhook送信先のチャンネル。デフォルトはこのチャンネル"
    )]
    pub channel: Option<String>,
    #[arg(
        short,
        long,
        help = "webhookの所有者。デフォルトはあなた一人。ユーザー1名、またはグループ1つを指定可能(予定)"
    )]
    pub owner: Option<String>,
}

#[derive(Debug, Clone, Args)]
pub struct WebhookList;

#[derive(Debug, Clone, Args)]
pub struct WebhookDelete {
    pub id: String,
}
