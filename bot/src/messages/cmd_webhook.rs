use futures::{pin_mut, StreamExt};
use indoc::formatdoc;
use uuid::Uuid;

use cli::webhook::complete::{Webhook, WebhookCreate, WebhookDelete, WebhookList};
use repository::{
    self, AllRepository, GroupMemberRepository, GroupRepository, OwnerRepository, UserRepository,
    WebhookRepository,
};
use traq_client::Client;

use super::{Bot, Result};

impl Bot {
    pub(super) async fn handle_webhook_command(
        &self,
        client: &impl Client,
        repo: &impl AllRepository,
        wh: Webhook,
    ) -> Result<()> {
        use Webhook::*;
        match wh {
            Create(create) => self.handle_webhook_create(client, repo, create).await,
            Delete(delete) => self.handle_webhook_delete(client, repo, delete).await,
            List(list) => self.handle_webhook_list(client, repo, list).await,
        }
    }

    async fn handle_webhook_create(
        &self,
        client: &impl Client,
        repo: &impl AllRepository,
        create: WebhookCreate,
    ) -> Result<()> {
        let owner = create.owner;

        // ownerには投稿者自身が含まれている必要がある
        let own_users = if owner.group {
            client
                .get_group_members(&owner.id)
                .await?
                .into_iter()
                .map(|gm| gm.id)
                .collect::<Vec<_>>()
        } else {
            vec![owner.id]
        };
        if !own_users.contains(&create.user_id) {
            let message = format!("エラー: --ownerに @{} が含まれていません", create.user_name);
            client
                .send_message(&create.talking_channel_id, &message, true)
                .await?;
            return Ok(());
        }

        // DBにユーザーとグループを追加
        let own_users = async_stream::stream! {
            for u in own_users {
                yield client.get_user(&u).await.map(|user| repository::User {
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
        repo.user_repository().create_ignore(&own_users).await?;
        repo.owner_repository()
            .create_ignore(&[owner.clone()])
            .await?;
        if owner.group {
            repo.group_repository()
                .create_ignore(&[repository::Group {
                    id: owner.id,
                    name: owner.name.clone(),
                }])
                .await?;
            let group_members = own_users
                .iter()
                .map(|u| repository::GroupMember {
                    group_id: owner.id,
                    user_id: u.id,
                })
                .collect::<Vec<_>>();
            repo.group_member_repository()
                .create_ignore(&group_members)
                .await?;
        }

        // webhook生成してDBに追加
        let mut id = Uuid::new_v4();
        while repo.webhook_repository().find(&id).await?.is_some() {
            // 重複しないようにする
            id = Uuid::new_v4();
        }
        let webhook = repository::Webhook {
            id,
            channel_id: create.channel_id,
            owner_id: owner.id,
        };
        repo.webhook_repository().create(webhook.clone()).await?;

        let message_title = if owner.group {
            format!(":@{}:によってWebhookが作成されました", create.user_name)
        } else {
            String::new()
        };
        let channel_path = if !create.channel_dm {
            client.get_channel_path(&webhook.channel_id).await?
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
        let msg = message.trim();
        let it = async_stream::stream! {
            for u in own_users {
                yield client.send_direct_message(&u.id, msg, true).await;
            }
        };
        pin_mut!(it);
        while let Some(r) = it.next().await {
            r?;
        }
        Ok(())
    }

    async fn handle_webhook_list(
        &self,
        client: &impl Client,
        repo: &impl AllRepository,
        list: WebhookList,
    ) -> Result<()> {
        let user_id = list.user_id;
        let groups = repo
            .group_member_repository()
            .filter_by_uid(&user_id)
            .await?;
        let owners: Vec<Uuid> = groups
            .into_iter()
            .map(|gm| gm.group_id)
            .chain([user_id].into_iter())
            .collect();
        let webhooks = repo.webhook_repository().filter_by_oids(&owners).await?;
        let code = serde_json::to_string_pretty(&webhooks)?;
        client.send_code_dm(&user_id, "json", &code).await?;
        Ok(())
    }

    async fn handle_webhook_delete(
        &self,
        client: &impl Client,
        repo: &impl AllRepository,
        delete: WebhookDelete,
    ) -> Result<()> {
        let webhook = repo.webhook_repository().find(&delete.webhook_id).await?;
        if webhook.is_none() {
            let message = format!("エラー: webhook {} は存在しません", delete.webhook_id);
            client
                .send_message(&delete.talking_channel_id, &message, true)
                .await?;
            return Ok(());
        }
        let webhook = webhook.unwrap();
        let owner = repo
            .owner_repository()
            .find(&webhook.owner_id)
            .await?
            .unwrap();
        let own_users = if owner.group {
            client
                .get_group_members(&owner.id)
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
            client
                .send_message(&delete.talking_channel_id, &message, true)
                .await?;
            return Ok(());
        }
        repo.webhook_repository().delete(&delete.webhook_id).await?;
        let it = async_stream::stream! {
            let message = format!("Webhook {} を削除しました", delete.webhook_id);
            for u in own_users {
                yield client.send_direct_message(&u, &message, false).await;
            }
        };
        pin_mut!(it);
        while let Some(r) = it.next().await {
            r?;
        }
        Ok(())
    }
}
