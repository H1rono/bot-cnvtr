use clap::Parser;

use traq_bot_http::payloads::{DirectMessageCreatedPayload, MessageCreatedPayload};

use super::{Bot, Result};
use crate::{
    cli::{CompletedCmds, Incomplete},
    Cli, Database,
};

mod cmd_webhook;

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
        _db: &Database,
    ) -> Result<()> {
        print!(
            "{}さんがダイレクトメッセージを投稿しました。\n内容: {}\n",
            payload.message.user.display_name, payload.message.text
        );
        let mut msg = payload.message.plain_text.trim().to_string();
        if !msg.starts_with('@') {
            msg = format!("@BOT_cnvtr {msg}");
        }
        msg = msg.replace('#', r"\#");
        let args = shlex::split(&msg).unwrap_or(vec![]);
        let cid = payload.message.channel_id;
        let cli = match Cli::try_parse_from(args.into_iter()) {
            Ok(c) => c,
            Err(e) => {
                self.send_code(&cid, "", &e.to_string()).await?;
                return Ok(());
            }
        };
        let cmd = cli.cmd.complete(payload.message);
        match cmd {
            CompletedCmds::Webhook(w) => self.handle_webhook_command(w).await,
        }?;
        Ok(())
    }
}
