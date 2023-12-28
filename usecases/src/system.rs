use traq_bot_http::payloads::{JoinedPayload, LeftPayload};

use domain::{Repository, TraqClient};

use super::{Bot, Result};

impl Bot {
    pub(super) async fn on_joined(
        &self,
        _repo: &impl Repository,
        _client: &impl TraqClient,
        payload: JoinedPayload,
    ) -> Result<()> {
        println!("チャンネル {} に参加しました。", payload.channel.name);
        Ok(())
    }

    pub(super) async fn on_left(
        &self,
        _repo: &impl Repository,
        _client: &impl TraqClient,
        payload: LeftPayload,
    ) -> Result<()> {
        println!("チャンネル {} から退出しました。", payload.channel.name);
        Ok(())
    }
}
