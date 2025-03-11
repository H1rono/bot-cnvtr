use clap::{Args, Subcommand};
use domain::OwnerKind;
use serde::{Deserialize, Serialize};
use traq_bot_http::payloads::{DirectMessageCreatedPayload, MessageCreatedPayload, types::Message};
use uuid::Uuid;

use domain::User;

use super::complete;
use crate::cli::Incomplete;

#[must_use]
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

#[must_use]
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
            .map_or(context.channel_id, |e| e.id);
        let owner_name = self.owner.clone().unwrap_or_else(|| user.name.clone());
        let embed = embeds.iter().find(|e| e.raw == owner_name);
        let owner_id = embed.map_or(user.id, |e| e.id);
        let owner_kind = embed
            .and_then(|e| (e.r#type == "group").then_some(OwnerKind::Group))
            .unwrap_or(OwnerKind::SingleUser);
        complete::WebhookCreate {
            user: User {
                id: context.user.id.into(),
                name: context.user.name.clone().into(),
            },
            in_dm,
            talking_channel_id: context.channel_id.into(),
            channel_name: self.channel.clone(),
            channel_id: channel_id.into(),
            channel_dm: self.channel.is_none() && in_dm,
            owner_id: owner_id.into(),
            owner_name,
            owner_kind,
        }
    }
}

impl<'a> Incomplete<&'a MessageCreatedPayload> for WebhookCreate {
    type Completed = complete::WebhookCreate;

    fn complete(&self, context: &'a MessageCreatedPayload) -> Self::Completed {
        self.complete((false, &context.message))
    }
}

impl<'a> Incomplete<&'a DirectMessageCreatedPayload> for WebhookCreate {
    type Completed = complete::WebhookCreate;

    fn complete(&self, context: &'a DirectMessageCreatedPayload) -> Self::Completed {
        self.complete((true, &context.message))
    }
}

#[must_use]
#[derive(Debug, Clone, Args, Deserialize, Serialize)]
pub struct WebhookList;

impl<'a> Incomplete<&'a Message> for WebhookList {
    type Completed = complete::WebhookList;

    fn complete(&self, context: &'a Message) -> Self::Completed {
        let user = User {
            id: context.user.id.into(),
            name: context.user.name.clone().into(),
        };
        complete::WebhookList { user }
    }
}

#[must_use]
#[derive(Debug, Clone, Args, Deserialize, Serialize)]
pub struct WebhookDelete {
    pub id: Uuid,
}

impl<'a> Incomplete<&'a Message> for WebhookDelete {
    type Completed = complete::WebhookDelete;

    fn complete(&self, context: &'a Message) -> Self::Completed {
        let user = User {
            id: context.user.id.into(),
            name: context.user.name.clone().into(),
        };
        complete::WebhookDelete {
            user,
            talking_channel_id: context.channel_id.into(),
            webhook_id: self.id.into(),
        }
    }
}
