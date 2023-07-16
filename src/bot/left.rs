use traq_bot_http::payloads::LeftPayload;

use crate::Database;

use super::{Bot, Error};

impl Bot {
    pub async fn on_left(&self, payload: LeftPayload, _db: &Database) -> Result<(), Error> {
        println!("チャンネル {} から退出しました。", payload.channel.name);
        Ok(())
    }
}
