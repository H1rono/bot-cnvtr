use clap::Parser;

use traq_bot_http::payloads::{DirectMessageCreatedPayload, MessageCreatedPayload};

use crate::{Cli, Database};

use super::{Bot, Error};

impl Bot {
    pub async fn on_message_created(
        &self,
        payload: MessageCreatedPayload,
        _db: &Database,
    ) -> Result<(), Error> {
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
    ) -> Result<(), Error> {
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
        let code = format!("{:?}", cli);
        self.send_code(&cid, "", &code).await?;
        Ok(())
    }
}
