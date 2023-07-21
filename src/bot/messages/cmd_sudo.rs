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

    async fn handle_sudo_wh_list_all(&self, _list_all: ListAll, _db: &Database) -> Result<()> {
        Ok(())
    }

    async fn handle_sudo_wh_delete(&self, _delete: Delete, _db: &Database) -> Result<()> {
        Ok(())
    }
}
