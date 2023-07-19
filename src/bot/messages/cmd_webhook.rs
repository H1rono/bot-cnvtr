use indoc::formatdoc;
use uuid::Uuid;

use super::{Bot, Result};

use crate::cli::webhook::complete::{Webhook, WebhookCreate, WebhookDelete, WebhookList};
use crate::Database;

impl Bot {
    pub(super) async fn handle_webhook_command(&self, wh: Webhook, db: &Database) -> Result<()> {
        use Webhook::*;
        match wh {
            Create(create) => self.handle_webhook_create(create, db).await,
            Delete(delete) => self.handle_webhook_delete(delete, db).await,
            List(list) => self.handle_webhook_list(list, db).await,
        }
    }

    async fn handle_webhook_create(&self, create: WebhookCreate, _db: &Database) -> Result<()> {
        let message = formatdoc! {
            r##"
                :@{}:の要望 -- Webhook作成
                チャンネルID: {}
                所有者: @{}
            "##,
            create.user_name,
            create.channel_id,
            create.owner.name
        };
        self.send_direct_message(&create.user_id, &message, true)
            .await?;
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

    async fn handle_webhook_delete(&self, _: WebhookDelete, _db: &Database) -> Result<()> {
        // TODO
        Ok(())
    }
}
