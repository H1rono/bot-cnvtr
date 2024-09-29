use futures::TryFutureExt;
use indoc::formatdoc;
use uuid::Uuid;

use domain::{Infra, Owner, OwnerKind, Repository, Result, TraqClient, User};

use super::BotImpl;
use crate::cli::webhook::complete::{Webhook, WebhookCreate, WebhookDelete, WebhookList};

impl BotImpl {
    pub(super) async fn handle_webhook_command<I>(&self, infra: &I, wh: Webhook) -> Result<()>
    where
        I: Infra,
    {
        use Webhook::{Create, Delete, List};
        match wh {
            Create(create) => self.handle_webhook_create(infra, create).await,
            Delete(delete) => self.handle_webhook_delete(infra, delete).await,
            List(list) => self.handle_webhook_list(infra, list).await,
        }
    }

    async fn handle_webhook_create<I>(&self, infra: &I, create: WebhookCreate) -> Result<()>
    where
        I: Infra,
    {
        let client = infra.traq_client();
        let repo = infra.repo();

        if create.channel_dm {
            let message = "エラー: Webhook投稿先にDMを指定することはできません";
            client
                .send_message(&create.talking_channel_id, message, false)
                .await?;
            return Ok(());
        }
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
                    name: create.owner_name.into(),
                };
                Owner::SingleUser(user)
            }
        };

        // ownerには投稿者自身が含まれている必要がある
        let owner_contain_self = match &owner {
            Owner::Group(g) => g.members.iter().any(|u| u.id == create.user.id),
            Owner::SingleUser(u) => u.id == create.user.id,
        };
        if !owner_contain_self {
            let message = format!(
                "エラー: --ownerに @{name} が含まれていません",
                name = create.user.name
            );
            client
                .send_message(&create.talking_channel_id, &message, true)
                .await?;
            return Ok(());
        }

        // webhook生成してDBに追加
        let id = Uuid::now_v7().into();
        let channel_id = create.channel_id;
        let webhook = domain::Webhook::new(id, channel_id, owner);
        repo.add_webhook(&webhook).await?;

        let message_title = match webhook.owner.kind() {
            OwnerKind::Group => format!(
                ":@{name}:によってWebhookが作成されました",
                name = create.user.name
            ),
            OwnerKind::SingleUser => String::from("Webhookが作成されました"),
        };
        let channel_path = client.get_channel_path(&webhook.channel_id).await?;
        let message = formatdoc! {
            r##"
                ### {message_title}

                Webhook ID: {id}
                投稿先チャンネル: {channel_path}
                各サービスに対応するWebhookエンドポイントは以下の通りです:

                - GitHub: https://cnvtr.trap.show/wh/{id}/github
                - Gitea: https://cnvtr.trap.show/wh/{id}/gitea
                - ClickUp: https://cnvtr.trap.show/wh/{id}/clickup

                Webhookを削除する場合は `@{bot_name} webhook delete {id}` と投稿してください
            "##,
            bot_name = &self.name,
            id = webhook.id,
        };
        let msg = message.trim();
        let own_users = webhook.owner.iter_users();
        let notifications = own_users.map(|u| client.send_direct_message(&u.id, msg, true));
        futures::future::try_join_all(notifications).await?;
        Ok(())
    }

    async fn handle_webhook_delete<I>(&self, infra: &I, delete: WebhookDelete) -> Result<()>
    where
        I: Infra,
    {
        let repo = infra.repo();
        let client = infra.traq_client();

        let Some(webhook) = repo.find_webhook(&delete.webhook_id).await? else {
            let message = format!(
                "エラー: webhook {id} は存在しません",
                id = delete.webhook_id
            );
            client
                .send_message(&delete.talking_channel_id, &message, true)
                .await?;
            return Ok(());
        };
        let own_users_contain_self = webhook.owner.iter_users().any(|u| u.id == delete.user.id);
        if !own_users_contain_self {
            let message = format!(
                "エラー: webhook所有者に @{name} が含まれていません",
                name = delete.user.name,
            );
            client
                .send_message(&delete.talking_channel_id, &message, true)
                .await?;
            return Ok(());
        }
        repo.remove_webhook(&webhook).await?;
        let own_users = webhook.owner.iter_users();
        let message = format!("Webhook {id} を削除しました", id = delete.webhook_id);
        let notifications = own_users.map(|u| client.send_direct_message(&u.id, &message, false));
        futures::future::try_join_all(notifications).await?;
        Ok(())
    }

    async fn handle_webhook_list<I>(&self, infra: &I, list: WebhookList) -> Result<()>
    where
        I: Infra,
    {
        let client = infra.traq_client();

        let webhooks = infra.repo().filter_webhook_by_user(&list.user).await?;
        let channel_paths = webhooks.iter().map(|w| {
            client
                .get_channel_path(&w.channel_id)
                .map_ok(move |c| (w, c))
        });
        let channel_paths = futures::future::try_join_all(channel_paths).await?;
        let message = channel_paths
            .into_iter()
            .map(|(w, c)| {
                formatdoc! {
                    r#"
                        Webhook ID: {id}
                        投稿先チャンネル: {c}
                    "#,
                    id = w.id
                }
            })
            .collect::<Vec<_>>()
            .join("\n---\n\n");
        client
            .send_direct_message(&list.user.id, &message, true)
            .await?;
        Ok(())
    }
}
