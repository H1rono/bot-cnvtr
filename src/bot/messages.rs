use clap::Parser;
use indoc::indoc;

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
        let args = shlex::split(&payload.message.plain_text).unwrap_or(vec![]);
        let cid = payload.message.channel_id;
        let cli = match Cli::try_parse_from(args.into_iter()) {
            Ok(c) => c,
            Err(e) => {
                let message = format!(
                    indoc! {r#"
                    ```
                    {}
                    ```
                "#},
                    e
                );
                self.send_message(&cid, &message, false).await?;
                return Ok(());
            }
        };
        let message = format!(
            indoc! {r#"
            ```
            {:?}
            ```
        "#},
            cli
        );
        self.send_message(&cid, &message, false).await?;
        Ok(())
    }
}
