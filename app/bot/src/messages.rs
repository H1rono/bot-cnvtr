use clap::Parser;
use traq_bot_http::payloads::{DirectMessageCreatedPayload, MessageCreatedPayload};

use domain::{Error, Infra, MessageId, Result, StampId, TraqClient};

use crate::cli::{Cli, CompletedCmds, Incomplete};
use crate::BotImpl;

mod cmd_sudo;
mod cmd_webhook;

impl BotImpl {
    fn parse_command(&self, cmd: &str) -> Result<Cli, clap::Error> {
        let cmd = cmd.trim().to_string();
        let cmd = (!cmd.starts_with('@'))
            .then(|| format!("@{} {}", &self.name, cmd))
            .unwrap_or(cmd)
            .replace('#', r"\#");
        let args = shlex::split(&cmd).unwrap_or_default();
        Cli::try_parse_from(args)
    }

    async fn run_command<I>(
        &self,
        infra: &I,
        message_id: &MessageId,
        cmd: CompletedCmds,
    ) -> Result<()>
    where
        I: Infra,
        Error: From<I::Error>,
    {
        use CompletedCmds::*;
        let client = infra.traq_client();
        let res = match cmd {
            Webhook(w) => self.handle_webhook_command(infra, w).await,
            Sudo(s) => self.handle_sudo_command(infra, s).await,
        };
        match res {
            Ok(_) => {
                // :done:
                const STAMP_ID: StampId =
                    StampId(uuid::uuid!("aea52f9a-7484-47ed-ab8f-3b4cc84a474d"));
                client.add_message_stamp(message_id, &STAMP_ID, 1).await?;
                Ok(())
            }
            Err(e) => {
                // :melting_face:
                const STAMP_ID: StampId =
                    StampId(uuid::uuid!("67c90d0e-18da-483e-9b2f-e6e50adec29d"));
                client.add_message_stamp(message_id, &STAMP_ID, 1).await?;
                Err(e)
            }
        }
    }

    pub(super) async fn on_message_created<I>(
        &self,
        infra: &I,
        payload: MessageCreatedPayload,
    ) -> Result<()>
    where
        I: Infra,
        Error: From<I::Error>,
    {
        tracing::info!("Message from {}", payload.message.user.display_name);
        let message = &payload.message;
        if message.user.bot {
            tracing::debug!("Ignore BOT");
            return Ok(());
        }
        let cli = match self.parse_command(&message.plain_text) {
            Ok(c) => c,
            Err(e) => {
                let channel_id = message.channel_id.into();
                infra
                    .traq_client()
                    .send_code(&channel_id, "", &e.to_string())
                    .await?;
                return Ok(());
            }
        };
        tracing::debug!(cli = ?cli);
        let mid = payload.message.id.into();
        let cmd = cli.cmd.complete(&payload);
        self.run_command(infra, &mid, cmd).await
    }

    pub(super) async fn on_direct_message_created<I>(
        &self,
        infra: &I,
        payload: DirectMessageCreatedPayload,
    ) -> Result<()>
    where
        I: Infra,
        Error: From<I::Error>,
    {
        tracing::info!("Direct Message from {}", payload.message.user.display_name);
        let message = &payload.message;
        if message.user.bot {
            tracing::debug!("Ignore BOT");
            return Ok(());
        }
        let cli = match self.parse_command(&message.plain_text) {
            Ok(c) => c,
            Err(e) => {
                let channel_id = message.channel_id.into();
                infra
                    .traq_client()
                    .send_code(&channel_id, "", &e.to_string())
                    .await?;
                return Ok(());
            }
        };
        tracing::debug!(cli = ?cli);
        let mid = payload.message.id.into();
        let cmd = cli.cmd.complete(&payload);
        self.run_command(infra, &mid, cmd).await
    }
}
