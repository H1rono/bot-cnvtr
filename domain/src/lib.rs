pub mod error;
mod event;
mod group;
mod id;
pub(crate) mod macros;
mod owner;
mod user;
mod webhook;

use std::future::Future;

use serde::{Deserialize, Serialize};

pub use error::{Error, Result};
pub use id::{ChannelId, GroupId, MessageId, OwnerId, StampId, UserId, WebhookId};

#[must_use]
#[derive(Clone, Debug)]
pub struct Event {
    pub channel_id: ChannelId,
    pub kind: String,
    pub body: String,
}

#[must_use]
pub trait EventSubscriber: Clone + Send + Sync + 'static {
    type Error: Into<crate::Error> + Send + Sync + 'static;

    fn send(&self, event: Event) -> impl Future<Output = Result<(), Self::Error>> + Send;
}

#[must_use]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub id: UserId,
    pub name: String,
}

#[must_use]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Group {
    pub id: GroupId,
    pub name: String,
    pub members: Vec<User>,
}

#[must_use]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Owner {
    Group(Group),
    SigleUser(User),
}

#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum OwnerKind {
    Group,
    SingleUser,
}

#[must_use]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Webhook {
    pub id: WebhookId,
    pub channel_id: ChannelId,
    pub owner: Owner,
}

#[must_use]
pub trait Repository: Send + Sync + 'static {
    type Error: Into<crate::error::Error> + Send + Sync + 'static;

    fn add_webhook(
        &self,
        webhook: &Webhook,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;
    fn remove_webhook(
        &self,
        webhook: &Webhook,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;
    fn list_webhooks(&self) -> impl Future<Output = Result<Vec<Webhook>, Self::Error>> + Send;
    fn find_webhook(
        &self,
        id: &WebhookId,
    ) -> impl Future<Output = Result<Option<Webhook>, Self::Error>> + Send;
    fn filter_webhook_by_owner(
        &self,
        owner: &Owner,
    ) -> impl Future<Output = Result<Vec<Webhook>, Self::Error>> + Send;
    fn filter_webhook_by_channel(
        &self,
        channel_id: &ChannelId,
    ) -> impl Future<Output = Result<Vec<Webhook>, Self::Error>> + Send;
    fn filter_webhook_by_user(
        &self,
        user: &User,
    ) -> impl Future<Output = Result<Vec<Webhook>, Self::Error>> + Send;
}

#[must_use]
pub trait TraqClient: Send + Sync + 'static {
    type Error: Into<crate::error::Error> + Send + Sync + 'static;

    fn send_message(
        &self,
        channel_id: &ChannelId,
        content: &str,
        embed: bool,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;

    fn send_code(
        &self,
        channel_id: &ChannelId,
        lang: &str,
        code: &str,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send {
        async move {
            let message = indoc::formatdoc! {
                r#"
                    ```{}
                    {}
                    ```
                "#,
                lang, code
            };
            self.send_message(channel_id, &message, false).await
        }
    }

    fn send_direct_message(
        &self,
        user_id: &UserId,
        content: &str,
        embed: bool,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;

    fn send_code_dm(
        &self,
        user_id: &UserId,
        lang: &str,
        code: &str,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send {
        async move {
            let message = indoc::formatdoc! {
                r#"
                    ```{}
                    {}
                    ```
                "#,
                lang, code
            };
            self.send_direct_message(user_id, &message, false).await
        }
    }

    fn get_group(
        &self,
        group_id: &GroupId,
    ) -> impl Future<Output = Result<Group, Self::Error>> + Send;

    fn get_user(&self, user_id: &UserId) -> impl Future<Output = Result<User, Self::Error>> + Send;

    fn get_channel_path(
        &self,
        channel_id: &ChannelId,
    ) -> impl Future<Output = Result<String, Self::Error>> + Send;

    fn add_message_stamp(
        &self,
        message_id: &MessageId,
        stamp_id: &StampId,
        count: i32,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;
}

#[must_use]
pub trait Infra: Send + Sync + 'static {
    type Error: Into<crate::error::Error> + Send + Sync + 'static;
    type Repo: Repository<Error = Self::Error>;
    type TClient: TraqClient<Error = Self::Error>;
    type ESub: EventSubscriber<Error = Self::Error>;

    fn repo(&self) -> &Self::Repo;
    fn traq_client(&self) -> &Self::TClient;
    fn event_subscriber(&self) -> &Self::ESub;
}
