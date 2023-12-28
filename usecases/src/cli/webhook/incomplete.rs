use clap::{Args, Subcommand};
use domain::OwnerKind;
use serde::{Deserialize, Serialize};
use traq_bot_http::payloads::{types::Message, DirectMessageCreatedPayload, MessageCreatedPayload};
use uuid::Uuid;

use domain::User;

use super::complete;
use crate::cli::Incomplete;

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
        let owner_name = self.owner.clone().unwrap_or_else(|| user.name.clone());
        let embed = embeds.iter().find(|e| e.raw == owner_name);
        let owner_id = embed.map(|e| e.id).unwrap_or(user.id);
        let owner_kind = if embed.map(|e| e.r#type == "group").unwrap_or_default() {
            OwnerKind::Group
        } else {
            OwnerKind::SingleUser
        };
        complete::WebhookCreate {
            user_id: context.user.id,
            user_name: context.user.name.clone(),
            in_dm,
            talking_channel_id: context.channel_id,
            channel_name: self.channel.clone(),
            channel_id,
            channel_dm: self.channel.is_none() && in_dm,
            owner_id,
            owner_name,
            owner_kind,
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
        let user = User {
            id: context.user.id,
            name: context.user.name.clone(),
        };
        complete::WebhookList { user }
    }
}

#[derive(Debug, Clone, Args, Deserialize, Serialize)]
pub struct WebhookDelete {
    pub id: Uuid,
}

impl<'a> Incomplete<&'a Message> for WebhookDelete {
    type Completed = complete::WebhookDelete;

    fn complete(&self, context: &'a Message) -> Self::Completed {
        let user = User {
            id: context.user.id,
            name: context.user.name.clone(),
        };
        complete::WebhookDelete {
            user,
            talking_channel_id: context.channel_id,
            webhook_id: self.id,
        }
    }
}
