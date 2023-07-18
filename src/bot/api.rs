use indoc::indoc;
use uuid::Uuid;

use traq::apis::group_api::get_user_group_members;
use traq::apis::message_api::post_message;
use traq::apis::user_api::post_direct_message;
use traq::models::{Message, PostMessageRequest, UserGroupMember};

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
        let channel_id = channel_id.to_string();
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

    pub async fn send_direct_message(
        &self,
        user_id: &Uuid,
        content: &str,
        embed: bool,
    ) -> Result<Message> {
        let req = PostMessageRequest {
            content: content.to_string(),
            embed: Some(embed),
        };
        let user_id = user_id.to_string();
        let res = post_direct_message(&self.config, &user_id, Some(req)).await?;
        Ok(res)
    }

    pub async fn send_code_dm(&self, user_id: &Uuid, lang: &str, code: &str) -> Result<Message> {
        let message = format!(
            indoc! {r#"
            ```{}
            {}
            ```
        "#},
            lang, code
        );
        self.send_direct_message(user_id, &message, false).await
    }

    pub async fn get_group_members(&self, group_id: &Uuid) -> Result<Vec<UserGroupMember>> {
        let group_id = group_id.to_string();
        let res = get_user_group_members(&self.config, &group_id).await?;
        Ok(res)
    }
}
