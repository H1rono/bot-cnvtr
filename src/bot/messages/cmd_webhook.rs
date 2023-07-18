use super::{Bot, Result};

use crate::cli::webhook::complete::{Webhook, WebhookCreate, WebhookDelete, WebhookList};

impl Bot {
    pub(super) async fn handle_webhook_command(&self, wh: Webhook) -> Result<()> {
        use Webhook::*;
        match wh {
            Create(create) => self.handle_webhook_create(create).await,
            Delete(delete) => self.handle_webhook_delete(delete).await,
            List(list) => self.handle_webhook_list(list).await,
        }
    }

    async fn handle_webhook_create(&self, create: WebhookCreate) -> Result<()> {
        let code = serde_json::to_string_pretty(&create)?;
        self.send_code(&create.channel_id, "json", &code).await?;
        Ok(())
    }

    async fn handle_webhook_list(&self, _: WebhookList) -> Result<()> {
        // TODO
        Ok(())
    }

    async fn handle_webhook_delete(&self, _: WebhookDelete) -> Result<()> {
        // TODO
        Ok(())
    }
}
