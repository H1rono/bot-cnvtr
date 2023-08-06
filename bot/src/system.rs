use traq_bot_http::payloads::{JoinedPayload, LeftPayload};

use model::Database;

use super::{Bot, Result};

impl Bot {
    pub async fn on_joined<Db: Database>(&self, payload: JoinedPayload, _db: &Db) -> Result<()> {
        println!("チャンネル {} に参加しました。", payload.channel.name);
        Ok(())
    }

    pub async fn on_left<Db: Database>(&self, payload: LeftPayload, _db: &Db) -> Result<()> {
        println!("チャンネル {} から退出しました。", payload.channel.name);
        Ok(())
    }
}
