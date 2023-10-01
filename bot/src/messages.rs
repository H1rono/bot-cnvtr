use clap::Parser;

use traq_bot_http::payloads::{types::Message, DirectMessageCreatedPayload, MessageCreatedPayload};

use cli::{Cli, CompletedCmds, Incomplete};
use repository::AllRepository;
use uuid::Uuid;

use super::{Bot, Result};

mod cmd_sudo;
mod cmd_webhook;

fn parse_command(cmd: &str) -> Result<Cli, clap::Error> {
    let cmd = cmd.trim().to_string();
    let cmd = (!cmd.starts_with('@'))
        .then(|| format!("@BOT_cnvtr {}", cmd))
        .unwrap_or(cmd)
        .replace('#', r"\#");
    let args = shlex::split(&cmd).unwrap_or_default();
    Cli::try_parse_from(args)
}

impl Bot {
    async fn parse(&self, message: &Message) -> Result<Option<Cli>> {
        match parse_command(&message.plain_text) {
            Ok(c) => Ok(Some(c)),
            Err(e) => {
                self.client
                    .send_code(&message.channel_id, "", &e.to_string())
                    .await?;
                Ok(None)
            }
        }
    }

    async fn run_command(
        &self,
        message_id: &Uuid,
        cmd: CompletedCmds,
        repo: &impl AllRepository,
    ) -> Result<()> {
        use CompletedCmds::*;
        let res = match cmd {
            Webhook(w) => self.handle_webhook_command(w, repo).await,
            Sudo(s) => self.handle_sudo_command(s, repo).await,
        };
        match res {
            Ok(_) => {
                // :done:
                const STAMP_ID: Uuid = uuid::uuid!("aea52f9a-7484-47ed-ab8f-3b4cc84a474d");
                self.client
                    .add_message_stamp(message_id, &STAMP_ID, 1)
                    .await?;
                Ok(())
            }
            Err(e) => {
                // :melting_face:
                const STAMP_ID: Uuid = uuid::uuid!("67c90d0e-18da-483e-9b2f-e6e50adec29d");
                self.client
                    .add_message_stamp(message_id, &STAMP_ID, 1)
                    .await?;
                Err(e)
            }
        }
    }

    pub async fn on_message_created(
        &self,
        payload: MessageCreatedPayload,
        repo: &impl AllRepository,
    ) -> Result<()> {
        print!(
            "{}さんがメッセージを投稿しました。\n内容: {}\n",
            payload.message.user.display_name, payload.message.text
        );
        let cli = match self.parse(&payload.message).await? {
            Some(c) => c,
            None => return Ok(()),
        };
        let mid = &payload.message.id;
        let cmd = cli.cmd.complete(&payload);
        self.run_command(mid, cmd, repo).await
    }

    pub async fn on_direct_message_created(
        &self,
        payload: DirectMessageCreatedPayload,
        repo: &impl AllRepository,
    ) -> Result<()> {
        print!(
            "{}さんがダイレクトメッセージを投稿しました。\n内容: {}\n",
            payload.message.user.display_name, payload.message.text
        );
        let cli = match self.parse(&payload.message).await? {
            Some(c) => c,
            None => return Ok(()),
        };
        let mid = &payload.message.id;
        let cmd = cli.cmd.complete(&payload);
        self.run_command(mid, cmd, repo).await
    }
}
