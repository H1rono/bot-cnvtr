use sqlx::migrate::Migrator;
use sqlx::MySqlPool;

use domain::{
    ChannelId, Failure, Group, GroupId, Owner, OwnerId, OwnerKind, Repository, User, Webhook,
    WebhookId,
};

pub(crate) mod model;
pub mod opt;

pub const MIGRATOR: Migrator = sqlx::migrate!("./migrations");

#[must_use]
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

    async fn collect_group_members(&self, gid: &GroupId) -> Result<Vec<User>, Failure> {
        use futures::TryFutureExt;

        let group_members = self.filter_group_members_by_gid(gid).await?;
        let members = group_members.iter().map(|gm| {
            self.find_user(&gm.user_id).map_ok(|u| {
                let crate::model::User { id, name } = u;
                User {
                    id,
                    name: name.into(),
                }
            })
        });
        let members = futures::future::try_join_all(members).await?;
        Ok(members)
    }

    async fn complete_webhook(&self, w: &crate::model::Webhook) -> Result<Webhook, Failure> {
        let o = self.find_owner(&w.owner_id).await?;
        let owner = if o.kind == OwnerKind::Group {
            let gid = o.id.0.into();
            let g = self.find_group(&gid).await?;
            let members = self.collect_group_members(&gid).await?;
            let group = Group {
                id: g.id,
                name: g.name.into(),
                members,
            };
            Owner::Group(group)
        } else {
            let uid = o.id.0.into();
            let u = self.find_user(&uid).await?;
            let user = User {
                id: u.id,
                name: u.name.into(),
            };
            Owner::SingleUser(user)
        };
        Ok(Webhook {
            id: w.id,
            channel_id: w.channel_id,
            owner,
        })
    }

    async fn complete_webhooks(
        &self,
        ws: &[crate::model::Webhook],
    ) -> Result<Vec<Webhook>, Failure> {
        let it = ws.iter().map(|w| self.complete_webhook(w));
        let webhooks = futures::future::try_join_all(it).await?;
        Ok(webhooks)
    }
}

impl Repository for RepositoryImpl {
    async fn add_webhook(&self, webhook: &Webhook) -> Result<(), Failure> {
        let w = crate::model::Webhook {
            id: webhook.id,
            channel_id: webhook.channel_id,
            owner_id: webhook.owner.id(),
        };
        self.create_webhook(w).await?;
        let o = crate::model::Owner {
            id: webhook.owner.id(),
            name: webhook.owner.name().to_string(),
            kind: webhook.owner.kind(),
        };
        // 既に存在するかもしれないのでcreate_ignoreで
        self.create_ignore_owners(&[o]).await?;
        match &webhook.owner {
            Owner::Group(group) => {
                use crate::model::Group;
                use crate::model::GroupMember;
                use crate::model::User;
                let g = Group {
                    id: group.id,
                    name: group.name.clone().into(),
                };
                self.create_ignore_groups(&[g]).await?;
                let gms = group
                    .members
                    .iter()
                    .map(|u| GroupMember {
                        user_id: u.id,
                        group_id: group.id,
                    })
                    .collect::<Vec<_>>();
                self.create_ignore_group_members(&gms).await?;
                let us = group
                    .members
                    .iter()
                    .map(|u| User {
                        id: u.id,
                        name: u.name.clone().into(),
                    })
                    .collect::<Vec<_>>();
                self.create_ignore_users(&us).await?;
            }
            Owner::SingleUser(user) => {
                let u = crate::model::User {
                    id: user.id,
                    name: user.name.clone().into(),
                };
                self.create_ignore_users(&[u]).await?;
            }
        }
        Ok(())
    }

    async fn remove_webhook(&self, webhook: &Webhook) -> Result<(), Failure> {
        self.delete_webhook(&webhook.id).await
    }

    async fn list_webhooks(&self) -> Result<Vec<Webhook>, Failure> {
        let ws = self.read_webhooks().await?;
        let webhooks = self.complete_webhooks(&ws).await?;
        Ok(webhooks)
    }

    async fn find_webhook(&self, id: &WebhookId) -> Result<Webhook, Failure> {
        let w = self.find_webhook(id).await?;
        let webhook = self.complete_webhook(&w).await?;
        Ok(webhook)
    }

    async fn filter_webhook_by_owner(&self, owner: &Owner) -> Result<Vec<Webhook>, Failure> {
        let ws = self.filter_webhooks_by_oid(owner.id()).await?;
        self.complete_webhooks(&ws).await
    }

    async fn filter_webhook_by_channel(
        &self,
        channel_id: &ChannelId,
    ) -> Result<Vec<Webhook>, Failure> {
        let ws = self.filter_webhooks_by_cid(*channel_id).await?;
        self.complete_webhooks(&ws).await
    }

    async fn filter_webhook_by_user(&self, user: &User) -> Result<Vec<Webhook>, Failure> {
        let gms = self.filter_group_members_by_uid(&user.id).await?;
        let mut oids = gms
            .into_iter()
            .map(|gm| gm.group_id.0.into())
            .collect::<Vec<OwnerId>>();
        oids.push(user.id.0.into());
        let ws = self.filter_webhooks_by_oids(&oids).await?;
        self.complete_webhooks(&ws).await
    }
}
