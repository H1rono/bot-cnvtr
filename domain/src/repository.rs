use std::future::Future;

use crate::owner::Owner;
use crate::user::User;
use crate::webhook::{ChannelId, Webhook, WebhookId};

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
