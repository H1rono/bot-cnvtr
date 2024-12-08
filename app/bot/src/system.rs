use traq_bot_http::payloads::{JoinedPayload, LeftPayload};

use domain::{Infra, Result};

use crate::BotImplInner;

impl BotImplInner {
    #[allow(clippy::unused_async)]
    pub(super) async fn on_joined(&self, _: &impl Infra, payload: JoinedPayload) -> Result<()> {
        tracing::info!("チャンネル {} に参加しました。", payload.channel.name);
        Ok(())
    }

    #[allow(clippy::unused_async)]
    pub(super) async fn on_left(&self, _: &impl Infra, payload: LeftPayload) -> Result<()> {
        tracing::info!("チャンネル {} から退出しました。", payload.channel.name);
        Ok(())
    }
}
