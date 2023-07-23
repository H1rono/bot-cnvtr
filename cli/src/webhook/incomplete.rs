use clap::{Args, Subcommand};
use serde::{Deserialize, Serialize};
use traq_bot_http::payloads::{types::Message, DirectMessageCreatedPayload, MessageCreatedPayload};
use uuid::Uuid;

use model::Owner;

use super::complete;
use crate::Incomplete;

#[derive(Debug, Clone, Subcommand)]
pub enum Webhook {
    Create(WebhookCreate),
    List(WebhookList),
    Delete(WebhookDelete),
}

impl<'a> Incomplete<&'a MessageCreatedPayload> for Webhook {
    type Completed = complete::Webhook;

    fn complete(&self, context: &'a MessageCreatedPayload) -> Self::Completed {
        match self {
            Self::Create(create) => complete::Webhook::Create(create.complete(context)),
            Self::List(list) => complete::Webhook::List(list.complete(context)),
            Self::Delete(delete) => complete::Webhook::Delete(delete.complete(context)),
        }
    }
}

impl<'a> Incomplete<&'a DirectMessageCreatedPayload> for Webhook {
    type Completed = complete::Webhook;

    fn complete(&self, context: &'a DirectMessageCreatedPayload) -> Self::Completed {
        match self {
            Self::Create(create) => complete::Webhook::Create(create.complete(context)),
            Self::List(list) => complete::Webhook::List(list.complete(context)),
            Self::Delete(delete) => complete::Webhook::Delete(delete.complete(context)),
        }
    }
}

#[derive(Debug, Clone, Args, Deserialize, Serialize)]
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

impl<'a> Incomplete<(bool, &'a Message)> for WebhookCreate {
    type Completed = complete::WebhookCreate;

    fn complete(&self, context: (bool, &'a Message)) -> Self::Completed {
        let (in_dm, context) = context;
        let user = &context.user;
        let embeds = &context.embedded;
        let channel_id = self
            .channel
            .as_deref()
            .and_then(|c| embeds.iter().find(|e| e.raw == c))
            .map(|e| e.id)
            .unwrap_or(context.channel_id);
        let owner = self
            .owner
            .as_deref()
            .and_then(|o| embeds.iter().find(|e| e.raw == o))
            .map(|e| Owner {
                id: e.id,
                name: e
                    .raw
                    .starts_with('@')
                    .then(|| e.raw.replacen('@', "", 1))
                    .unwrap_or(e.raw.clone()),
                group: e.type_ == "group",
            })
            .unwrap_or(Owner {
                id: user.id,
                name: user.name.clone(),
                group: false,
            });
        complete::WebhookCreate {
            user_id: context.user.id,
            user_name: context.user.name.clone(),
            in_dm,
            talking_channel_id: context.channel_id,
            channel_name: self.channel.clone(),
            channel_id,
            channel_dm: self.channel.is_none() && in_dm,
            owner,
        }
    }
}

impl<'a> Incomplete<&'a MessageCreatedPayload> for WebhookCreate {
    type Completed = complete::WebhookCreate;

    fn complete(&self, context: &'a MessageCreatedPayload) -> Self::Completed {
        self.complete((true, &context.message))
    }
}

impl<'a> Incomplete<&'a DirectMessageCreatedPayload> for WebhookCreate {
    type Completed = complete::WebhookCreate;

    fn complete(&self, context: &'a DirectMessageCreatedPayload) -> Self::Completed {
        self.complete((true, &context.message))
    }
}

#[derive(Debug, Clone, Args, Deserialize, Serialize)]
pub struct WebhookList;

impl<'a> Incomplete<&'a Message> for WebhookList {
    type Completed = complete::WebhookList;

    fn complete(&self, context: &'a Message) -> Self::Completed {
        complete::WebhookList {
            user_id: context.user.id,
        }
    }
}

#[derive(Debug, Clone, Args, Deserialize, Serialize)]
pub struct WebhookDelete {
    pub id: Uuid,
}

impl<'a> Incomplete<&'a Message> for WebhookDelete {
    type Completed = complete::WebhookDelete;

    fn complete(&self, context: &'a Message) -> Self::Completed {
        complete::WebhookDelete {
            user_id: context.user.id,
            user_name: context.user.name.clone(),
            talking_channel_id: context.channel_id,
            webhook_id: self.id,
        }
    }
}
