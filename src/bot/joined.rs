use traq_bot_http::payloads::JoinedPayload;

use crate::Database;

use super::{Bot, Error};

impl Bot {
    pub async fn on_joined(&self, payload: JoinedPayload, _db: &Database) -> Result<(), Error> {
        println!("チャンネル {} に参加しました。", payload.channel.name);
        Ok(())
    }
}
