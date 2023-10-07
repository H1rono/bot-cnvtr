use crate::cli::sudo::{
    webhook::{Completed, Delete, ListAll},
    SudoCompleted,
};
use futures::{pin_mut, StreamExt};

use super::{Bot, Error, Result};
use crate::traits::{Repository, TraqClient};

impl Bot {
    pub(super) async fn handle_sudo_command<E1, E2>(
        &self,
        repo: &impl Repository<Error = E1>,
        client: &impl TraqClient<Error = E2>,
        sudo: SudoCompleted,
    ) -> Result<()>
    where
        Error: From<E1> + From<E2>,
    {
        use SudoCompleted::*;
        match sudo {
            Webhook(Completed::ListAll(list_all)) => {
                self.handle_sudo_wh_list_all(repo, client, list_all).await
            }
            Webhook(Completed::Delete(delete)) => {
                self.handle_sudo_wh_delete(repo, client, delete).await
            }
        }
    }

    async fn handle_sudo_wh_list_all<E1, E2>(
        &self,
        repo: &impl Repository<Error = E1>,
        client: &impl TraqClient<Error = E2>,
        list_all: ListAll,
    ) -> Result<()>
    where
        Error: From<E1> + From<E2>,
    {
        let webhooks = repo.list_webhooks().await?;
        let code = serde_json::to_string_pretty(&webhooks)?;
        client
            .send_code_dm(&list_all.user_id, "json", &code)
            .await?;
        Ok(())
    }

    async fn handle_sudo_wh_delete<E1, E2>(
        &self,
        repo: &impl Repository<Error = E1>,
        client: &impl TraqClient<Error = E2>,
        delete: Delete,
    ) -> Result<()>
    where
        Error: From<E1> + From<E2>,
    {
        if !delete.valid {
            let message = "Permission denied.";
            client
                .send_code(&delete.talking_channel_id, "", message)
                .await?;
        }
        let webhook = match repo.find_webhook(&delete.id).await? {
            Some(w) => w,
            None => {
                let message = format!("エラー: webhook {} は存在しません", delete.id);
                client
                    .send_message(&delete.talking_channel_id, &message, false)
                    .await?;
                return Ok(());
            }
        };
        let owner = &webhook.owner;
        let own_users = match owner {
            entity::Owner::Group(g) => g.members.iter().collect(),
            entity::Owner::SigleUser(u) => vec![u],
        };
        repo.remove_webhook(&webhook).await?;
        let it = async_stream::stream! {
            let message = format!("Webhook {} を削除しました", delete.id);
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
}
