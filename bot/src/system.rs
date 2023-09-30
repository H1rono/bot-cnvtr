use traq_bot_http::payloads::{JoinedPayload, LeftPayload};

use repository::AllRepository;

use super::{Bot, Result};

impl Bot {
    pub async fn on_joined(
        &self,
        payload: JoinedPayload,
        _repo: &impl AllRepository,
    ) -> Result<()> {
        println!("チャンネル {} に参加しました。", payload.channel.name);
        Ok(())
    }

    pub async fn on_left(&self, payload: LeftPayload, _repo: &impl AllRepository) -> Result<()> {
        println!("チャンネル {} から退出しました。", payload.channel.name);
        Ok(())
    }
}
