use super::{Bot, Result};

use cli::sudo::{
    webhook::{Completed, Delete, ListAll},
    SudoCompleted,
};
use futures::{pin_mut, StreamExt};
use model::Database;

impl Bot {
    pub(super) async fn handle_sudo_command<Db: Database>(
        &self,
        sudo: SudoCompleted,
        db: &Db,
    ) -> Result<()> {
        use SudoCompleted::*;
        match sudo {
            Webhook(Completed::ListAll(list_all)) => {
                self.handle_sudo_wh_list_all(list_all, db).await
            }
            Webhook(Completed::Delete(delete)) => self.handle_sudo_wh_delete(delete, db).await,
        }
    }

    async fn handle_sudo_wh_list_all<Db: Database>(
        &self,
        list_all: ListAll,
        db: &Db,
    ) -> Result<()> {
        if !list_all.valid {
            let message = "Permission denied.";
            self.send_code(&list_all.talking_channel_id, "", message)
                .await?;
        }
        let webhooks = db.read_webhooks().await?;
        let code = serde_json::to_string_pretty(&webhooks)?;
        self.send_code_dm(&list_all.user_id, "json", &code).await?;
        Ok(())
    }

    async fn handle_sudo_wh_delete<Db: Database>(&self, delete: Delete, db: &Db) -> Result<()> {
        if !delete.valid {
            let message = "Permission denied.";
            self.send_code(&delete.talking_channel_id, "", message)
                .await?;
        }
        let webhook = match db.find_webhook(&delete.id).await? {
            Some(w) => w,
            None => {
                let message = format!("エラー: webhook {} は存在しません", delete.id);
                self.send_message(&delete.talking_channel_id, &message, false)
                    .await?;
                return Ok(());
            }
        };
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
        db.delete_webhook(&webhook.id).await?;
        let it = async_stream::stream! {
            let message = format!("Webhook {} を削除しました", delete.id);
            for u in own_users {
                yield self.send_direct_message(&u, &message, false).await;
            }
        };
        pin_mut!(it);
        while let Some(r) = it.next().await {
            r?;
        }
        Ok(())
    }
}
