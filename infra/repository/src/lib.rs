use sqlx::migrate::Migrator;
use sqlx::MySqlPool;

use domain::{ChannelId, Group, Owner, OwnerKind, Repository, User, Webhook, WebhookId};

pub mod error;
pub(crate) mod model;
pub mod opt;

pub const MIGRATOR: Migrator = sqlx::migrate!("./migrations");

pub struct RepositoryImpl(pub(crate) MySqlPool);

impl RepositoryImpl {
    pub fn new(pool: MySqlPool) -> Self {
        Self(pool)
    }

    pub async fn connect(url: &str) -> sqlx::Result<Self> {
        let pool = MySqlPool::connect(url).await?;
        Ok(Self::new(pool))
    }

    pub async fn migrate(&self) -> sqlx::Result<()> {
        MIGRATOR.run(&self.0).await?;
        Ok(())
    }

    async fn complete_webhook(&self, w: &crate::model::Webhook) -> Result<Webhook, sqlx::Error> {
        let o = self.find_owner(&w.owner_id).await?.unwrap();
        let owner = if o.group {
            let g = self.find_group(&o.id).await?.unwrap();
            let gms = self.filter_group_members_by_gid(&g.id).await?;
            let mut members = vec![];
            for gm in gms {
                let u = self.find_user(&gm.user_id).await?.unwrap();
                members.push(User {
                    id: u.id.into(),
                    name: u.name,
                });
            }
            let group = Group {
                id: g.id.into(),
                name: g.name,
                members,
            };
            Owner::Group(group)
        } else {
            let u = self.find_user(&o.id).await?.unwrap();
            let user = User {
                id: u.id.into(),
                name: u.name,
            };
            Owner::SigleUser(user)
        };
        Ok(Webhook {
            id: w.id.into(),
            channel_id: w.channel_id.into(),
            owner,
        })
    }

    async fn complete_webhooks(
        &self,
        ws: &[crate::model::Webhook],
    ) -> Result<Vec<Webhook>, sqlx::Error> {
        let mut webhooks = vec![];
        for w in ws {
            let webhook = self.complete_webhook(w).await?;
            webhooks.push(webhook);
        }
        Ok(webhooks)
    }
}

impl Repository for RepositoryImpl {
    type Error = crate::error::Error;

    async fn add_webhook(&self, webhook: &Webhook) -> Result<(), Self::Error> {
        let w = crate::model::Webhook {
            id: webhook.id.into(),
            channel_id: webhook.channel_id.into(),
            owner_id: webhook.owner.id().into(),
        };
        self.create_webhook(w).await?;
        let o = crate::model::Owner {
            id: webhook.owner.id().into(),
            name: webhook.owner.name().to_string(),
            group: webhook.owner.kind() == OwnerKind::Group,
        };
        // 既に存在するかもしれないのでcreate_ignoreで
        self.create_ignore_owners(&[o]).await?;
        match &webhook.owner {
            Owner::Group(group) => {
                use crate::model::Group;
                use crate::model::GroupMember;
                use crate::model::User;
                let g = Group {
                    id: group.id.into(),
                    name: group.name.clone(),
                };
                self.create_ignore_groups(&[g]).await?;
                let gms = group
                    .members
                    .iter()
                    .map(|u| GroupMember {
                        user_id: u.id.into(),
                        group_id: group.id.into(),
                    })
                    .collect::<Vec<_>>();
                self.create_ignore_group_members(&gms).await?;
                let us = group
                    .members
                    .iter()
                    .map(|u| User {
                        id: u.id.into(),
                        name: u.name.clone(),
                    })
                    .collect::<Vec<_>>();
                self.create_ignore_users(&us).await?;
            }
            Owner::SigleUser(user) => {
                let u = crate::model::User {
                    id: user.id.into(),
                    name: user.name.clone(),
                };
                self.create_ignore_users(&[u]).await?;
            }
        }
        Ok(())
    }

    async fn remove_webhook(&self, webhook: &Webhook) -> Result<(), Self::Error> {
        let webhook_id = webhook.id.into();
        Ok(self.delete_webhook(&webhook_id).await?)
    }

    async fn list_webhooks(&self) -> Result<Vec<Webhook>, Self::Error> {
        let ws = self.read_webhooks().await?;
        let mut webhooks = vec![];
        for w in ws {
            let webhook = self.complete_webhook(&w).await?;
            webhooks.push(webhook);
        }
        Ok(webhooks)
    }

    async fn find_webhook(&self, id: &WebhookId) -> Result<Option<Webhook>, Self::Error> {
        let w = self.find_webhook(&id.0).await?;
        if let Some(w) = w {
            let webhook = self.complete_webhook(&w).await?;
            Ok(Some(webhook))
        } else {
            Ok(None)
        }
    }

    async fn filter_webhook_by_owner(&self, owner: &Owner) -> Result<Vec<Webhook>, Self::Error> {
        let ws = self.filter_webhooks_by_oid(owner.id().0).await?;
        Ok(self.complete_webhooks(&ws).await?)
    }

    async fn filter_webhook_by_channel(
        &self,
        channel_id: &ChannelId,
    ) -> Result<Vec<Webhook>, Self::Error> {
        let ws = self.filter_webhooks_by_cid(channel_id.0).await?;
        Ok(self.complete_webhooks(&ws).await?)
    }

    async fn filter_webhook_by_user(&self, user: &User) -> Result<Vec<Webhook>, Self::Error> {
        let gms = self.filter_group_members_by_uid(&user.id.0).await?;
        let mut oids = gms.into_iter().map(|gm| gm.group_id).collect::<Vec<_>>();
        oids.push(user.id.0);
        let ws = self.filter_webhooks_by_oids(&oids).await?;
        Ok(self.complete_webhooks(&ws).await?)
    }
}
