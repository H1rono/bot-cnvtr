use indoc::indoc;
use uuid::Uuid;

use traq::apis::message_api::post_message;
use traq::models::{Message, PostMessageRequest};

use super::{Bot, Result};

impl Bot {
    pub async fn send_message(
        &self,
        channel_id: &Uuid,
        content: &str,
        embed: bool,
    ) -> Result<Message> {
        let req = PostMessageRequest {
            content: content.to_string(),
            embed: Some(embed),
        };
        let channel_id = format!("{}", channel_id);
        let res = post_message(&self.config, &channel_id, Some(req)).await?;
        Ok(res)
    }

    pub async fn send_code(&self, channel_id: &Uuid, lang: &str, code: &str) -> Result<Message> {
        let message = format!(
            indoc! {r#"
            ```{}
            {}
            ```
        "#},
            lang, code
        );
        self.send_message(channel_id, &message, false).await
    }
}
