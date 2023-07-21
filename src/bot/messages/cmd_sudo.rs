use super::{Bot, Result};

use crate::cli::sudo::SudoCompleted;
use crate::Database;

impl Bot {
    pub(super) async fn handle_sudo_command(
        &self,
        sudo: SudoCompleted,
        _db: &Database,
    ) -> Result<()> {
        use SudoCompleted::*;
        match sudo {
            Webhook(_wh) => Ok(()),
        }
    }
}
