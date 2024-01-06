use traq_bot_http::payloads::{JoinedPayload, LeftPayload};

use domain::{Infra, Result};

use crate::BotImpl;

impl BotImpl {
    pub(super) async fn on_joined(
        &self,
        _infra: &impl Infra,
        payload: JoinedPayload,
    ) -> Result<()> {
        // TODO: tracing
        println!("チャンネル {} に参加しました。", payload.channel.name);
        Ok(())
    }

    pub(super) async fn on_left(&self, _infra: &impl Infra, payload: LeftPayload) -> Result<()> {
        // TODO: tracing
        println!("チャンネル {} から退出しました。", payload.channel.name);
        Ok(())
    }
}
