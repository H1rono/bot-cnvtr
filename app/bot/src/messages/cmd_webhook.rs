use futures::{pin_mut, StreamExt};
use indoc::formatdoc;
use uuid::Uuid;

use domain::{Error, Infra, Owner, OwnerKind, Repository, Result, TraqClient, User};

use super::BotImpl;
use crate::cli::webhook::complete::{Webhook, WebhookCreate, WebhookDelete, WebhookList};

impl BotImpl {
    pub(super) async fn handle_webhook_command<I>(&self, infra: &I, wh: Webhook) -> Result<()>
    where
        I: Infra,
        Error: From<I::Error>,
    {
        use Webhook::*;
        match wh {
            Create(create) => self.handle_webhook_create(infra, create).await,
            Delete(delete) => self.handle_webhook_delete(infra, delete).await,
            List(list) => self.handle_webhook_list(infra, list).await,
        }
    }

    async fn handle_webhook_create<I>(&self, infra: &I, create: WebhookCreate) -> Result<()>
    where
        I: Infra,
        Error: From<I::Error>,
    {
        let client = infra.traq_client();
        let repo = infra.repo();

        let owner = match create.owner_kind {
            OwnerKind::Group => {
                let group_id = create.owner_id.0.into();
                let group = client.get_group(&group_id).await?;
                Owner::Group(group)
            }
            OwnerKind::SingleUser => {
                let user_id = create.owner_id.0.into();
                let user = User {
                    id: user_id,
                    name: create.owner_name,
                };
                Owner::SigleUser(user)
            }
        };

        // ownerには投稿者自身が含まれている必要がある
        let owner_contain_self = match &owner {
            Owner::Group(g) => g.members.iter().any(|u| u.id == create.user.id),
            Owner::SigleUser(u) => u.id == create.user.id,
        };
        if !owner_contain_self {
            let message = format!("エラー: --ownerに @{} が含まれていません", create.user.name);
            client
                .send_message(&create.talking_channel_id, &message, true)
                .await?;
            return Ok(());
        }

        // webhook生成してDBに追加
        let mut id = Uuid::new_v4().into();
        while repo.find_webhook(&id).await?.is_some() {
            // 重複しないようにする
            id = Uuid::new_v4().into();
        }
        let channel_id = create.channel_id;
        let webhook = domain::Webhook::new(id, channel_id, owner);
        repo.add_webhook(&webhook).await?;

        let message_title = match webhook.owner.kind() {
            OwnerKind::Group => format!(":@{}:によってWebhookが作成されました", create.user.name),
            OwnerKind::SingleUser => String::from("Webhookが作成されました"),
        };
        let channel_path = if !create.channel_dm {
            client.get_channel_path(&webhook.channel_id).await?
        } else {
            "DM".to_string()
        };
        let message = formatdoc! {
            r##"
                ### {}

                Webhook ID: {}
                投稿先チャンネル: {}
                各サービスに対応するWebhookエンドポイントは以下の通りです:

                - GitHub: https://cnvtr.trap.show/wh/{}/github
                - Gitea: https://cnvtr.trap.show/wh/{}/gitea
                - ClickUp: https://cnvtr.trap.show/wh/{}/clickup

                Webhookを削除する場合は `@BOT_cnvtr webhook delete {}` と投稿してください
            "##,
            message_title,
            webhook.id,
            channel_path,
            webhook.id, webhook.id, webhook.id,
            webhook.id
        };
        let msg = message.trim();
        let own_users = webhook.owner.users();
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

    async fn handle_webhook_delete<I>(&self, infra: &I, delete: WebhookDelete) -> Result<()>
    where
        I: Infra,
        Error: From<I::Error>,
    {
        let repo = infra.repo();
        let client = infra.traq_client();

        let Some(webhook) = repo.find_webhook(&delete.webhook_id).await? else {
            let message = format!("エラー: webhook {} は存在しません", delete.webhook_id);
            client
                .send_message(&delete.talking_channel_id, &message, true)
                .await?;
            return Ok(());
        };
        let own_users_contain_self = webhook.owner.users().iter().any(|u| u.id == delete.user.id);
        if !own_users_contain_self {
            let message = format!(
                "エラー: webhook所有者に @{} が含まれていません",
                delete.user.name,
            );
            client
                .send_message(&delete.talking_channel_id, &message, true)
                .await?;
            return Ok(());
        }
        repo.remove_webhook(&webhook).await?;
        let own_users = webhook.owner.users();
        let it = async_stream::stream! {
            let message = format!("Webhook {} を削除しました", delete.webhook_id);
            for u in own_users {
                yield client.send_direct_message(&u.id, &message, false).await;
            }
        };
        pin_mut!(it);
        while let Some(r) = it.next().await {
            r?;
        }
        Ok(())
    }

    async fn handle_webhook_list<I>(&self, infra: &I, list: WebhookList) -> Result<()>
    where
        I: Infra,
        Error: From<I::Error>,
    {
        let client = infra.traq_client();

        let webhooks = infra.repo().filter_webhook_by_user(&list.user).await?;
        let mut serialized_webhooks = vec![];
        for w in webhooks {
            let channel_path = client.get_channel_path(&w.channel_id).await?;
            let ser_w = formatdoc! {
                r#"
                    Webhook ID: {}
                    投稿先チャンネル: {}
                "#,
                w.id, channel_path
            };
            serialized_webhooks.push(ser_w);
        }
        let message = serialized_webhooks.join("\n\n---\n\n");
        client
            .send_direct_message(&list.user.id, &message, false)
            .await?;
        Ok(())
    }
}
