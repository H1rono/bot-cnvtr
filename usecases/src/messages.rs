use clap::Parser;
use traq_bot_http::payloads::{DirectMessageCreatedPayload, MessageCreatedPayload};
use uuid::Uuid;

use domain::{Repository, TraqClient};

use super::{Bot, Error, Result};
use crate::cli::{Cli, CompletedCmds, Incomplete};

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
    async fn run_command2<E1, E2>(
        &self,
        repo: &impl Repository<Error = E1>,
        client: &impl TraqClient<Error = E2>,
        message_id: &Uuid,
        cmd: CompletedCmds,
    ) -> Result<()>
    where
        Error: From<E1> + From<E2>,
    {
        use CompletedCmds::*;
        let res = match cmd {
            Webhook(w) => self.handle_webhook_command(repo, client, w).await,
            Sudo(s) => self.handle_sudo_command(repo, client, s).await,
        };
        match res {
            Ok(_) => {
                // :done:
                const STAMP_ID: Uuid = uuid::uuid!("aea52f9a-7484-47ed-ab8f-3b4cc84a474d");
                client.add_message_stamp(message_id, &STAMP_ID, 1).await?;
                Ok(())
            }
            Err(e) => {
                // :melting_face:
                const STAMP_ID: Uuid = uuid::uuid!("67c90d0e-18da-483e-9b2f-e6e50adec29d");
                client.add_message_stamp(message_id, &STAMP_ID, 1).await?;
                Err(e)
            }
        }
    }

    pub(super) async fn on_message_created<E1, E2>(
        &self,
        repo: &impl Repository<Error = E1>,
        client: &impl TraqClient<Error = E2>,
        payload: MessageCreatedPayload,
    ) -> Result<()>
    where
        Error: From<E1> + From<E2>,
    {
        print!(
            "{}さんがメッセージを投稿しました。\n内容: {}\n",
            payload.message.user.display_name, payload.message.text
        );
        let message = &payload.message;
        let cli = match parse_command(&message.plain_text) {
            Ok(c) => c,
            Err(e) => {
                client
                    .send_code(&message.channel_id, "", &e.to_string())
                    .await?;
                return Ok(());
            }
        };
        let mid = &payload.message.id;
        let cmd = cli.cmd.complete(&payload);
        self.run_command2(repo, client, mid, cmd).await
    }

    pub(super) async fn on_direct_message_created<E1, E2>(
        &self,
        repo: &impl Repository<Error = E1>,
        client: &impl TraqClient<Error = E2>,
        payload: DirectMessageCreatedPayload,
    ) -> Result<()>
    where
        Error: From<E1> + From<E2>,
    {
        print!(
            "{}さんがダイレクトメッセージを投稿しました。\n内容: {}\n",
            payload.message.user.display_name, payload.message.text
        );
        let message = &payload.message;
        let cli = match parse_command(&message.plain_text) {
            Ok(c) => c,
            Err(e) => {
                client
                    .send_code(&message.channel_id, "", &e.to_string())
                    .await?;
                return Ok(());
            }
        };
        let mid = &payload.message.id;
        let cmd = cli.cmd.complete(&payload);
        self.run_command2(repo, client, mid, cmd).await
    }
}
