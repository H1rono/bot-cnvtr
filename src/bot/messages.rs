use clap::Parser;

use traq_bot_http::payloads::{DirectMessageCreatedPayload, MessageCreatedPayload};

use super::{Bot, Result};
use crate::{
    cli::{CompletedCmds, Incomplete},
    Cli, Database,
};

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
    pub async fn on_message_created(
        &self,
        payload: MessageCreatedPayload,
        _db: &Database,
    ) -> Result<()> {
        print!(
            "{}さんがメッセージを投稿しました。\n内容: {}\n",
            payload.message.user.display_name, payload.message.text
        );
        Ok(())
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
        let msg = &payload.message.plain_text;
        let cid = payload.message.channel_id;
        let cli = match parse_command(msg) {
            Ok(c) => c,
            Err(e) => {
                self.send_code(&cid, "", &e.to_string()).await?;
                return Ok(());
            }
        };
        let cmd = cli.cmd.complete(&payload);
        match cmd {
            CompletedCmds::Webhook(w) => self.handle_webhook_command(w, db).await,
        }?;
        Ok(())
    }
}
