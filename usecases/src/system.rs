use traq_bot_http::payloads::{JoinedPayload, LeftPayload};

use repository::AllRepository;
use traq_client::Client;

use super::{Bot, Result};

impl Bot {
    pub async fn on_joined(
        &self,
        _client: &impl Client,
        _repo: &impl AllRepository,
        payload: JoinedPayload,
    ) -> Result<()> {
        println!("チャンネル {} に参加しました。", payload.channel.name);
        Ok(())
    }

    pub async fn on_left(
        &self,
        _client: &impl Client,
        _repo: &impl AllRepository,
        payload: LeftPayload,
    ) -> Result<()> {
        println!("チャンネル {} から退出しました。", payload.channel.name);
        Ok(())
    }
}
