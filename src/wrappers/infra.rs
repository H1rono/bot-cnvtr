use serde::{Deserialize, Serialize};

use domain::{
    ChannelId, Event, EventSubscriber, GroupId, MessageId, Repository, StampId, TraqClient, UserId,
    WebhookId,
};
use repository::opt;

#[must_use]
#[derive(Clone)]
pub struct EventSubWrapper<S: EventSubscriber + Clone>(pub S);

impl<S: EventSubscriber + Clone> EventSubscriber for EventSubWrapper<S> {
    async fn send(&self, event: Event) -> Result<(), domain::Error> {
        self.0.send(event).await
    }
}

#[must_use]
pub struct RepoWrapper<R: Repository>(pub R);

impl<R: Repository> Repository for RepoWrapper<R> {
    async fn add_webhook(&self, webhook: &domain::Webhook) -> Result<(), domain::Error> {
        self.0.add_webhook(webhook).await
    }

    async fn remove_webhook(&self, webhook: &domain::Webhook) -> Result<(), domain::Error> {
        self.0.remove_webhook(webhook).await
    }

    async fn list_webhooks(&self) -> Result<Vec<domain::Webhook>, domain::Error> {
        self.0.list_webhooks().await
    }

    async fn find_webhook(&self, id: &WebhookId) -> Result<Option<domain::Webhook>, domain::Error> {
        self.0.find_webhook(id).await
    }

    async fn filter_webhook_by_owner(
        &self,
        owner: &domain::Owner,
    ) -> Result<Vec<domain::Webhook>, domain::Error> {
        self.0.filter_webhook_by_owner(owner).await
    }

    async fn filter_webhook_by_channel(
        &self,
        channel_id: &ChannelId,
    ) -> Result<Vec<domain::Webhook>, domain::Error> {
        self.0.filter_webhook_by_channel(channel_id).await
    }

    async fn filter_webhook_by_user(
        &self,
        user: &domain::User,
    ) -> Result<Vec<domain::Webhook>, domain::Error> {
        self.0.filter_webhook_by_user(user).await
    }
}

#[must_use]
pub struct TraqClientWrapper<C: TraqClient>(pub C);

impl<C> TraqClient for TraqClientWrapper<C>
where
    C: TraqClient,
{
    async fn send_message(
        &self,
        channel_id: &ChannelId,
        content: &str,
        embed: bool,
    ) -> Result<(), domain::Error> {
        self.0.send_message(channel_id, content, embed).await
    }

    async fn send_code(
        &self,
        channel_id: &ChannelId,
        lang: &str,
        code: &str,
    ) -> Result<(), domain::Error> {
        self.0.send_code(channel_id, lang, code).await
    }

    async fn send_direct_message(
        &self,
        user_id: &UserId,
        content: &str,
        embed: bool,
    ) -> Result<(), domain::Error> {
        self.0.send_direct_message(user_id, content, embed).await
    }

    async fn send_code_dm(
        &self,
        user_id: &UserId,
        lang: &str,
        code: &str,
    ) -> Result<(), domain::Error> {
        self.0.send_code_dm(user_id, lang, code).await
    }

    async fn get_group(&self, group_id: &GroupId) -> Result<domain::Group, domain::Error> {
        self.0.get_group(group_id).await
    }

    async fn get_user(&self, user_id: &UserId) -> Result<domain::User, domain::Error> {
        self.0.get_user(user_id).await
    }

    async fn get_channel_path(&self, channel_id: &ChannelId) -> Result<String, domain::Error> {
        self.0.get_channel_path(channel_id).await
    }

    async fn add_message_stamp(
        &self,
        message_id: &MessageId,
        stamp_id: &StampId,
        count: i32,
    ) -> Result<(), domain::Error> {
        self.0.add_message_stamp(message_id, stamp_id, count).await
    }
}

#[must_use]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RepoConfig {
    pub database: String,
    pub hostname: String,
    pub password: String,
    pub port: String,
    pub user: String,
}

impl RepoConfig {
    pub fn from_env() -> envy::Result<Self> {
        envy::prefixed("NS_MARIADB_")
            .from_env()
            .or_else(|_| envy::prefixed("MYSQL_").from_env())
    }

    #[must_use]
    pub fn database_url(&self) -> String {
        let Self {
            database,
            hostname,
            password,
            port,
            user,
        } = self;
        format!("mysql://{user}:{password}@{hostname}:{port}/{database}")
    }
}

impl TryFrom<RepoConfig> for opt::Opt {
    type Error = <u16 as std::str::FromStr>::Err;

    fn try_from(value: RepoConfig) -> Result<Self, Self::Error> {
        let RepoConfig {
            database,
            hostname,
            password,
            port,
            user,
        } = value;
        Ok(opt::Opt {
            hostname,
            user,
            password,
            port: port.parse()?,
            database,
        })
    }
}

#[must_use]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraqClientConfig {
    pub bot_access_token: String,
}
