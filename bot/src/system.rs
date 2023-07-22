use traq_bot_http::payloads::{JoinedPayload, LeftPayload};

use model::Database;

use super::{Bot, Result};

impl Bot {
    pub async fn on_joined(&self, payload: JoinedPayload, _db: &Database) -> Result<()> {
        println!("チャンネル {} に参加しました。", payload.channel.name);
        Ok(())
    }

    pub async fn on_left(&self, payload: LeftPayload, _db: &Database) -> Result<()> {
        println!("チャンネル {} から退出しました。", payload.channel.name);
        Ok(())
    }
}
