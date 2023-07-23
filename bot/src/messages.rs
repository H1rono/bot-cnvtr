use clap::Parser;

use traq_bot_http::payloads::{types::Message, DirectMessageCreatedPayload, MessageCreatedPayload};

use cli::{Cli, CompletedCmds, Incomplete};
use model::Database;

use super::{Bot, Result};

mod cmd_sudo;
mod cmd_webhook;

fn parse_command(cmd: &str) -> Result<Cli, clap::Error> {
    let cmd = cmd.trim().to_string();
    let cmd = (!cmd.starts_with('@'))
        .then(|| format!("@BOT_cnvtr {}", cmd))
        .unwrap_or(cmd)
        .replace('#', r"\#");
    let args = shlex::split(&cmd).unwrap_or(vec![]);
    Cli::try_parse_from(args.into_iter())
}

impl Bot {
    async fn parse(&self, message: &Message) -> Result<Option<Cli>> {
        match parse_command(&message.plain_text) {
            Ok(c) => Ok(Some(c)),
            Err(e) => {
                self.send_code(&message.channel_id, "", &e.to_string())
                    .await?;
                Ok(None)
            }
        }
    }

    async fn run_command(&self, cmd: CompletedCmds, db: &Database) -> Result<()> {
        use CompletedCmds::*;
        match cmd {
            Webhook(w) => self.handle_webhook_command(w, db).await,
            Sudo(s) => self.handle_sudo_command(s, db).await,
        }
    }

    pub async fn on_message_created(
        &self,
        payload: MessageCreatedPayload,
        db: &Database,
    ) -> Result<()> {
        print!(
            "{}さんがメッセージを投稿しました。\n内容: {}\n",
            payload.message.user.display_name, payload.message.text
        );
        let cli = match self.parse(&payload.message).await? {
            Some(c) => c,
            None => return Ok(()),
        };
        let cmd = cli.cmd.complete(&payload);
        self.run_command(cmd, db).await
    }

    pub async fn on_direct_message_created(
        &self,
        payload: DirectMessageCreatedPayload,
        db: &Database,
    ) -> Result<()> {
        print!(
            "{}さんがダイレクトメッセージを投稿しました。\n内容: {}\n",
            payload.message.user.display_name, payload.message.text
        );
        let cli = match self.parse(&payload.message).await? {
            Some(c) => c,
            None => return Ok(()),
        };
        let cmd = cli.cmd.complete(&payload);
        self.run_command(cmd, db).await
    }
}
