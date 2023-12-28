use async_trait::async_trait;
use uuid::Uuid;

use crate::owner::Owner;
use crate::user::User;
use crate::webhook::Webhook;

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

// pub trait WebhookHandler: Clone + Send + Sync + 'static {
//     type Error;

//     fn handle<'a>(
//         &self,
//         headers: impl Iterator<Item = (&'a str, &'a str)>,
//         payload: Value,
//     ) -> Result<Option<String>, Self::Error>;
// }
