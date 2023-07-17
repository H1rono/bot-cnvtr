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
}
