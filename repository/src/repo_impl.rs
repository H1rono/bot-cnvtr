use async_trait::async_trait;
use sqlx::MySqlPool;
use uuid::Uuid;

use entity::{Group, Owner, OwnerKind, User, Webhook};
use usecases::traits::Repository;

use crate::config::Config;
use crate::MIGRATOR;
use crate::{
    GroupMemberRepository, GroupRepository, OwnerRepository, UserRepository, WebhookRepository,
};

pub struct RepositoryImpl(pub(crate) MySqlPool);

impl RepositoryImpl {
    pub fn new(pool: MySqlPool) -> Self {
        Self(pool)
    }

    pub async fn from_config(c: Config) -> sqlx::Result<Self> {
        let url = c.database_url();
        let pool = MySqlPool::connect(&url).await?;
        Ok(Self::new(pool))
    }

    pub async fn migrate(&self) -> sqlx::Result<()> {
        MIGRATOR.run(&self.0).await?;
        Ok(())
    }

    async fn complete_webhook(
        &self,
        w: &crate::model::webhook::Webhook,
    ) -> Result<Webhook, sqlx::Error> {
        let o = <Self as OwnerRepository>::find(self, &w.owner_id)
            .await?
            .unwrap();
        let owner = if o.group {
            let g = <Self as GroupRepository>::find(self, &o.id).await?.unwrap();
            let gms = <Self as GroupMemberRepository>::filter_by_gid(self, &g.id).await?;
            let mut members = vec![];
            for gm in gms {
                let u = <Self as UserRepository>::find(self, &gm.user_id)
                    .await?
                    .unwrap();
                members.push(User {
                    id: u.id,
                    name: u.name,
                });
            }
            let group = Group {
                id: g.id,
                name: g.name,
                members,
            };
            Owner::Group(group)
        } else {
            let u = <Self as UserRepository>::find(self, &o.id).await?.unwrap();
            let user = User {
                id: u.id,
                name: u.name,
            };
            Owner::SigleUser(user)
        };
        Ok(Webhook {
            id: w.id,
            channel_id: w.channel_id,
            owner,
        })
    }

    async fn complete_webhooks(
        &self,
        ws: &[crate::model::webhook::Webhook],
    ) -> Result<Vec<Webhook>, sqlx::Error> {
        let mut webhooks = vec![];
        for w in ws {
            let webhook = self.complete_webhook(w).await?;
            webhooks.push(webhook);
        }
        Ok(webhooks)
    }
}

impl AsRef<MySqlPool> for RepositoryImpl {
    fn as_ref(&self) -> &MySqlPool {
        &self.0
    }
}

#[async_trait]
impl Repository for RepositoryImpl {
    type Error = sqlx::Error;

    async fn add_webhook(&self, webhook: &Webhook) -> Result<(), Self::Error> {
        let w = crate::model::webhook::Webhook {
            id: webhook.id,
            channel_id: webhook.channel_id,
            owner_id: webhook.owner.id(),
        };
        <Self as WebhookRepository>::create(self, w).await?;
        let o = crate::model::owner::Owner {
            id: webhook.owner.id(),
            name: webhook.owner.name().to_string(),
            group: webhook.owner.kind() == OwnerKind::Group,
        };
        // 既に存在するかもしれないのでcreate_ignoreで
        <Self as OwnerRepository>::create_ignore(self, &[o]).await?;
        match &webhook.owner {
            Owner::Group(group) => {
                use crate::model::group::Group;
                use crate::model::group_member::GroupMember;
                use crate::model::user::User;
                let g = Group {
                    id: group.id,
                    name: group.name.clone(),
                };
                <Self as GroupRepository>::create_ignore(self, &[g]).await?;
                let gms = group
                    .members
                    .iter()
                    .map(|u| GroupMember {
                        user_id: u.id,
                        group_id: group.id,
                    })
                    .collect::<Vec<_>>();
                <Self as GroupMemberRepository>::create_ignore(self, &gms).await?;
                let us = group
                    .members
                    .iter()
                    .map(|u| User {
                        id: u.id,
                        name: u.name.clone(),
                    })
                    .collect::<Vec<_>>();
                <Self as UserRepository>::create_ignore(self, &us).await?;
            }
            Owner::SigleUser(user) => {
                let u = crate::model::user::User {
                    id: user.id,
                    name: user.name.clone(),
                };
                <Self as UserRepository>::create_ignore(self, &[u]).await?;
            }
        }
        Ok(())
    }

    async fn remove_webhook(&self, webhook: &Webhook) -> Result<(), Self::Error> {
        <Self as WebhookRepository>::delete(self, &webhook.id).await
    }

    async fn list_webhooks(&self) -> Result<Vec<Webhook>, Self::Error> {
        let ws = <Self as WebhookRepository>::read(self).await?;
        let mut webhooks = vec![];
        for w in ws {
            let webhook = self.complete_webhook(&w).await?;
            webhooks.push(webhook);
        }
        Ok(webhooks)
    }

    async fn find_webhook(&self, id: &Uuid) -> Result<Option<Webhook>, Self::Error> {
        let w = <Self as WebhookRepository>::find(self, id).await?;
        if let Some(w) = w {
            let webhook = self.complete_webhook(&w).await?;
            Ok(Some(webhook))
        } else {
            Ok(None)
        }
    }

    async fn filter_webhook_by_owner(&self, owner: &Owner) -> Result<Vec<Webhook>, Self::Error> {
        let ws = <Self as WebhookRepository>::filter_by_oid(self, owner.id()).await?;
        self.complete_webhooks(&ws).await
    }

    async fn filter_webhook_by_channel(
        &self,
        channel_id: &Uuid,
    ) -> Result<Vec<Webhook>, Self::Error> {
        let ws = <Self as WebhookRepository>::filter_by_cid(self, *channel_id).await?;
        self.complete_webhooks(&ws).await
    }

    async fn filter_webhook_by_user(&self, user: &User) -> Result<Vec<Webhook>, Self::Error> {
        let gms = <Self as GroupMemberRepository>::filter_by_uid(self, &user.id).await?;
        let mut oids = gms.into_iter().map(|gm| gm.group_id).collect::<Vec<_>>();
        oids.push(user.id);
        let ws = <Self as WebhookRepository>::filter_by_oids(self, &oids).await?;
        self.complete_webhooks(&ws).await
    }
}
