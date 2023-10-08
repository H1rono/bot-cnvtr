use async_trait::async_trait;
use serde_json::value::Value;
use uuid::Uuid;

use entity::{Group, Owner, User, Webhook};

#[async_trait]
pub trait Repository: Send + Sync + 'static {
    type Error: Send + Sync + 'static;

    async fn add_webhook(&self, webhook: &Webhook) -> Result<(), Self::Error>;
    async fn remove_webhook(&self, webhook: &Webhook) -> Result<(), Self::Error>;
    async fn list_webhooks(&self) -> Result<Vec<Webhook>, Self::Error>;
    async fn find_webhook(&self, id: &Uuid) -> Result<Option<Webhook>, Self::Error>;
    async fn filter_webhook_by_owner(&self, owner: &Owner) -> Result<Vec<Webhook>, Self::Error>;
    async fn filter_webhook_by_channel(
        &self,
        channel_id: &Uuid,
    ) -> Result<Vec<Webhook>, Self::Error>;
    async fn filter_webhook_by_user(&self, user: &User) -> Result<Vec<Webhook>, Self::Error>;
}

#[async_trait]
pub trait TraqClient: Send + Sync + 'static {
    type Error: Send + Sync + 'static;

    async fn send_message(
        &self,
        channel_id: &Uuid,
        content: &str,
        embed: bool,
    ) -> Result<(), Self::Error>;

    async fn send_code(&self, channel_id: &Uuid, lang: &str, code: &str)
        -> Result<(), Self::Error>;

    async fn send_direct_message(
        &self,
        user_id: &Uuid,
        content: &str,
        embed: bool,
    ) -> Result<(), Self::Error>;

    async fn send_code_dm(&self, user_id: &Uuid, lang: &str, code: &str)
        -> Result<(), Self::Error>;

    async fn get_group(&self, group_id: &Uuid) -> Result<Group, Self::Error>;

    async fn get_user(&self, user_id: &Uuid) -> Result<User, Self::Error>;

    async fn get_channel_path(&self, channel_id: &Uuid) -> Result<String, Self::Error>;

    async fn add_message_stamp(
        &self,
        message_id: &Uuid,
        stamp_id: &Uuid,
        count: i32,
    ) -> Result<(), Self::Error>;
}

pub trait WebhookHandler: Clone + Send + Sync + 'static {
    type Error;

    fn handle<'a>(
        &self,
        headers: impl Iterator<Item = (&'a str, &'a str)>,
        payload: Value,
    ) -> Result<Option<String>, Self::Error>;
}
