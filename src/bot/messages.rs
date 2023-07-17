use traq_bot_http::payloads::{DirectMessageCreatedPayload, MessageCreatedPayload};

use crate::Database;

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
        Ok(())
    }
}
