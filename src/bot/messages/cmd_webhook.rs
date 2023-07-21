use futures::{pin_mut, StreamExt};
use indoc::formatdoc;
use uuid::Uuid;

use super::{Bot, Result};

use crate::cli::webhook::complete::{Webhook, WebhookCreate, WebhookDelete, WebhookList};
use crate::{model, Database};

impl Bot {
    pub(super) async fn handle_webhook_command(&self, wh: Webhook, db: &Database) -> Result<()> {
        use Webhook::*;
        match wh {
            Create(create) => self.handle_webhook_create(create, db).await,
            Delete(delete) => self.handle_webhook_delete(delete, db).await,
            List(list) => self.handle_webhook_list(list, db).await,
        }
    }

    async fn handle_webhook_create(&self, create: WebhookCreate, db: &Database) -> Result<()> {
        let owner = create.owner;

        // ownerには投稿者自身が含まれている必要がある
        let own_users = if owner.group {
            self.get_group_members(&owner.id)
                .await?
                .into_iter()
                .map(|gm| gm.id)
                .collect::<Vec<_>>()
        } else {
            vec![owner.id]
        };
        if !own_users.contains(&create.user_id) {
            let message = format!("エラー: --ownerに @{} が含まれていません", create.user_name);
            self.send_message(&create.talking_channel_id, &message, true)
                .await?;
            return Ok(());
        }

        // DBにユーザーとグループを追加
        let own_users = async_stream::stream! {
            for u in own_users {
                yield self.get_user(&u).await.map(|user| model::User {
                    id: user.id,
                    name: user.name,
                });
            }
        };
        pin_mut!(own_users);
        let own_users = own_users
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?;
        db.create_ignore_users(&own_users).await?;
        db.create_ignore_owners(&[owner.clone()]).await?;
        if owner.group {
            let group_members = own_users
                .iter()
                .map(|u| model::GroupMember {
                    group_id: owner.id,
                    user_id: u.id,
                })
                .collect::<Vec<_>>();
            db.create_ignore_group_members(&group_members).await?;
            db.create_ignore_groups(&[model::Group {
                id: owner.id,
                name: owner.name.clone(),
            }])
            .await?;
        }

        // webhook生成してDBに追加
        let mut id = sha256::digest(format!("{}/{}", owner.id, create.channel_id));
        while db.find_webhook(&id).await?.is_some() {
            id = sha256::digest(id);
        }
        let webhook = model::Webhook {
            id,
            channel_id: create.channel_id,
            owner_id: owner.id,
        };
        db.create_webhook(webhook.clone()).await?;

        let message_title = if owner.group {
            format!(":@{}:によってWebhookが作成されました", create.user_name)
        } else {
            String::new()
        };
        let channel_path = if !create.in_dm {
            self.get_channel_path(&webhook.channel_id).await?
        } else {
            "DM".to_string()
        };
        let message = formatdoc! {
            r##"
                {}
                投稿先チャンネル: {}
                各サービスに対応するWebhookエンドポイントは以下の通りです:
                - GitHub: https://cnvtr.trap.show/wh/{}/github
                - Gitea: https://cnvtr.trap.show/wh/{}/gitea
                - ClickUp: https://cnvtr.trap.show/wh/{}/clickup
            "##,
            message_title,
            channel_path,
            &webhook.id, &webhook.id, &webhook.id
        };
        let it = async_stream::stream! {
            for u in own_users {
                yield self.send_direct_message(&u.id, &message, true).await;
            }
        };
        pin_mut!(it);
        while let Some(r) = it.next().await {
            r?;
        }
        Ok(())
    }

    async fn handle_webhook_list(&self, list: WebhookList, db: &Database) -> Result<()> {
        let user_id = list.user_id;
        let groups = db.filter_group_member_by_uid(&user_id).await?;
        let owners: Vec<Uuid> = groups
            .into_iter()
            .map(|gm| gm.group_id)
            .chain([user_id].into_iter())
            .collect();
        let webhooks = db.filter_webhooks_by_oids(&owners).await?;
        let code = serde_json::to_string_pretty(&webhooks)?;
        self.send_code_dm(&user_id, "json", &code).await?;
        Ok(())
    }

    async fn handle_webhook_delete(&self, delete: WebhookDelete, db: &Database) -> Result<()> {
        let webhook = db.find_webhook(&delete.webhook_id).await?;
        if webhook.is_none() {
            let message = format!("エラー: webhook {} は存在しません", delete.webhook_id);
            self.send_message(&delete.talking_channel_id, &message, true)
                .await?;
            return Ok(());
        }
        let webhook = webhook.unwrap();
        let owner = db.find_owner(&webhook.owner_id).await?.unwrap();
        let own_users = if owner.group {
            self.get_group_members(&owner.id)
                .await?
                .into_iter()
                .map(|gm| gm.id)
                .collect::<Vec<_>>()
        } else {
            vec![owner.id]
        };
        if !own_users.contains(&delete.user_id) {
            let message = format!(
                "エラー: webhook所有者に @{} が含まれていません",
                delete.user_name
            );
            self.send_message(&delete.talking_channel_id, &message, true)
                .await?;
            return Ok(());
        }
        db.delete_webhook(&delete.webhook_id).await?;
        let message = format!("Webhook {} を削除しました", delete.webhook_id);
        self.send_message(&delete.talking_channel_id, &message, true)
            .await?;
        Ok(())
    }
}
