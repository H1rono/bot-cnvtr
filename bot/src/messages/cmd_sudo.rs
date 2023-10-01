use cli::sudo::{
    webhook::{Completed, Delete, ListAll},
    SudoCompleted,
};
use futures::{pin_mut, StreamExt};

use repository::{AllRepository, OwnerRepository, WebhookRepository};
use traq_client::Client;

use super::{Bot, Result};

impl Bot {
    pub(super) async fn handle_sudo_command(
        &self,
        client: &impl Client,
        repo: &impl AllRepository,
        sudo: SudoCompleted,
    ) -> Result<()> {
        use SudoCompleted::*;
        match sudo {
            Webhook(Completed::ListAll(list_all)) => {
                self.handle_sudo_wh_list_all(client, repo, list_all).await
            }
            Webhook(Completed::Delete(delete)) => {
                self.handle_sudo_wh_delete(client, repo, delete).await
            }
        }
    }

    async fn handle_sudo_wh_list_all(
        &self,
        client: &impl Client,
        repo: &impl AllRepository,
        list_all: ListAll,
    ) -> Result<()> {
        if !list_all.valid {
            let message = "Permission denied.";
            client
                .send_code(&list_all.talking_channel_id, "", message)
                .await?;
        }
        let webhooks = repo.webhook_repository().read().await?;
        let code = serde_json::to_string_pretty(&webhooks)?;
        client
            .send_code_dm(&list_all.user_id, "json", &code)
            .await?;
        Ok(())
    }

    async fn handle_sudo_wh_delete(
        &self,
        client: &impl Client,
        repo: &impl AllRepository,
        delete: Delete,
    ) -> Result<()> {
        if !delete.valid {
            let message = "Permission denied.";
            client
                .send_code(&delete.talking_channel_id, "", message)
                .await?;
        }
        let webhook = match repo.webhook_repository().find(&delete.id).await? {
            Some(w) => w,
            None => {
                let message = format!("エラー: webhook {} は存在しません", delete.id);
                client
                    .send_message(&delete.talking_channel_id, &message, false)
                    .await?;
                return Ok(());
            }
        };
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
        repo.webhook_repository().delete(&delete.id).await?;
        let it = async_stream::stream! {
            let message = format!("Webhook {} を削除しました", delete.id);
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
