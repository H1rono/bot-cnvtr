use domain::{ChannelId, Repository, WebhookId};

pub struct RepoWrapper<R: Repository>(pub R);

impl<R: Repository> Repository for RepoWrapper<R>
where
    domain::Error: From<R::Error>,
{
    type Error = domain::Error;

    async fn add_webhook(&self, webhook: &domain::Webhook) -> Result<(), Self::Error> {
        Ok(self.0.add_webhook(webhook).await?)
    }

    async fn remove_webhook(&self, webhook: &domain::Webhook) -> Result<(), Self::Error> {
        Ok(self.0.remove_webhook(webhook).await?)
    }

    async fn list_webhooks(&self) -> Result<Vec<domain::Webhook>, Self::Error> {
        Ok(self.0.list_webhooks().await?)
    }

    async fn find_webhook(&self, id: &WebhookId) -> Result<Option<domain::Webhook>, Self::Error> {
        Ok(self.0.find_webhook(id).await?)
    }

    async fn filter_webhook_by_owner(
        &self,
        owner: &domain::Owner,
    ) -> Result<Vec<domain::Webhook>, Self::Error> {
        Ok(self.0.filter_webhook_by_owner(owner).await?)
    }

    async fn filter_webhook_by_channel(
        &self,
        channel_id: &ChannelId,
    ) -> Result<Vec<domain::Webhook>, Self::Error> {
        Ok(self.0.filter_webhook_by_channel(channel_id).await?)
    }

    async fn filter_webhook_by_user(
        &self,
        user: &domain::User,
    ) -> Result<Vec<domain::Webhook>, Self::Error> {
        Ok(self.0.filter_webhook_by_user(user).await?)
    }
}
