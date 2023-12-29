use std::future::Future;

use uuid::Uuid;

use crate::owner::Owner;
use crate::user::User;
use crate::webhook::Webhook;

pub trait Repository: Send + Sync + 'static {
    type Error: Send + Sync + 'static;

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
        id: &Uuid,
    ) -> impl Future<Output = Result<Option<Webhook>, Self::Error>> + Send;
    fn filter_webhook_by_owner(
        &self,
        owner: &Owner,
    ) -> impl Future<Output = Result<Vec<Webhook>, Self::Error>> + Send;
    fn filter_webhook_by_channel(
        &self,
        channel_id: &Uuid,
    ) -> impl Future<Output = Result<Vec<Webhook>, Self::Error>> + Send;
    fn filter_webhook_by_user(
        &self,
        user: &User,
    ) -> impl Future<Output = Result<Vec<Webhook>, Self::Error>> + Send;
}
