use traq_bot_http::payloads::{JoinedPayload, LeftPayload};

use domain::{Infra, Result};

use crate::BotImpl;

impl BotImpl {
    pub(super) async fn on_joined(
        &self,
        _infra: &impl Infra,
        payload: JoinedPayload,
    ) -> Result<()> {
        tracing::info!("チャンネル {} に参加しました。", payload.channel.name);
        Ok(())
    }

    pub(super) async fn on_left(&self, _infra: &impl Infra, payload: LeftPayload) -> Result<()> {
        tracing::info!("チャンネル {} から退出しました。", payload.channel.name);
        Ok(())
    }
}
