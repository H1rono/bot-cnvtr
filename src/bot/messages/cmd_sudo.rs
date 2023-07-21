use super::{Bot, Result};

use crate::cli::sudo::{
    webhook::{Completed, Delete, ListAll},
    SudoCompleted,
};
use crate::Database;

impl Bot {
    pub(super) async fn handle_sudo_command(
        &self,
        sudo: SudoCompleted,
        db: &Database,
    ) -> Result<()> {
        use SudoCompleted::*;
        match sudo {
            Webhook(Completed::ListAll(list_all)) => {
                self.handle_sudo_wh_list_all(list_all, db).await
            }
            Webhook(Completed::Delete(delete)) => self.handle_sudo_wh_delete(delete, db).await,
        }
    }

    async fn handle_sudo_wh_list_all(&self, list_all: ListAll, _db: &Database) -> Result<()> {
        if !list_all.valid {
            let message = "Permission denied.";
            self.send_code(&list_all.talking_channel_id, "", message)
                .await?;
        }
        Ok(())
    }

    async fn handle_sudo_wh_delete(&self, delete: Delete, _db: &Database) -> Result<()> {
        if !delete.valid {
            let message = "Permission denied.";
            self.send_code(&delete.talking_channel_id, "", message)
                .await?;
        }
        Ok(())
    }
}
