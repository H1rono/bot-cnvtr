use anyhow::Context;

use domain::{Failure, Infra, Repository, TraqClient};

use super::BotImplInner;
use crate::cli::sudo::{
    webhook::{Completed, Delete, ListAll},
    SudoCompleted,
};
use crate::error::Error;

impl BotImplInner {
    pub(super) async fn handle_sudo_command<I>(
        &self,
        infra: &I,
        sudo: SudoCompleted,
    ) -> Result<(), Error>
    where
        I: Infra,
    {
        use SudoCompleted::Webhook;
        match sudo {
            Webhook(Completed::ListAll(list_all)) => {
                self.handle_sudo_wh_list_all(infra, list_all).await
            }
            Webhook(Completed::Delete(delete)) => self.handle_sudo_wh_delete(infra, delete).await,
        }
    }

    async fn handle_sudo_wh_list_all<I>(&self, infra: &I, list_all: ListAll) -> Result<(), Error>
    where
        I: Infra,
    {
        if !list_all.valid {
            let message = "Permission denied.";
            infra
                .traq_client()
                .send_code(&list_all.talking_channel_id, "", message)
                .await?;
            return Ok(());
        }
        let webhooks = infra.repo().list_webhooks().await?;
        let code = serde_json::to_string_pretty(&webhooks)
            .with_context(|| format!("failed to format {:?}", &webhooks))?;
        infra
            .traq_client()
            .send_code_dm(&list_all.user_id, "json", &code)
            .await?;
        Ok(())
    }

    async fn handle_sudo_wh_delete<I>(&self, infra: &I, delete: Delete) -> Result<(), Error>
    where
        I: Infra,
    {
        let repo = infra.repo();
        let client = infra.traq_client();

        if !delete.valid {
            let message = "Permission denied.";
            client
                .send_code(&delete.talking_channel_id, "", message)
                .await?;
            return Ok(());
        }
        let webhook = match repo.find_webhook(&delete.id).await {
            Ok(w) => w,
            Err(Failure::Reject(r)) => {
                tracing::warn!(reject = %r);
                let message = format!("エラー: {r}");
                client
                    .send_message(&delete.talking_channel_id, &message, false)
                    .await?;
                return Ok(());
            }
            Err(Failure::Error(e)) => return Err(e.into()),
        };
        let own_users = webhook.owner.iter_users();
        repo.remove_webhook(&webhook).await?;

        let message = format!("Webhook {id} を削除しました", id = delete.id);
        let notifications = own_users.map(|u| client.send_direct_message(&u.id, &message, false));
        futures::future::try_join_all(notifications).await?;

        Ok(())
    }
}
