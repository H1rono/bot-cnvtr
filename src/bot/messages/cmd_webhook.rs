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
        let code = serde_json::to_string_pretty(&create)?;
        self.send_code_dm(&create.user_id, "json", &code).await?;
        Ok(())
    }

    async fn handle_webhook_list(&self, _list: WebhookList, _db: &Database) -> Result<()> {
        // TODO
        Ok(())
    }

    async fn handle_webhook_delete(&self, _: WebhookDelete, _db: &Database) -> Result<()> {
        // TODO
        Ok(())
    }
}
