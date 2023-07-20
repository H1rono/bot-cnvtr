use async_recursion::async_recursion;
use indoc::formatdoc;
use uuid::Uuid;

use traq::apis::channel_api::get_channel;
use traq::apis::group_api::get_user_group_members;
use traq::apis::message_api::post_message;
use traq::apis::user_api::{get_user, post_direct_message};
use traq::models::{Message, PostMessageRequest, UserDetail, UserGroupMember};

use super::{Bot, Result};

impl Bot {
    pub async fn send_message(
        &self,
        channel_id: &Uuid,
        content: &str,
        embed: bool,
    ) -> Result<Message> {
        println!("send_message: {}", channel_id);
        let req = PostMessageRequest {
            content: content.to_string(),
            embed: Some(embed),
        };
        let channel_id = channel_id.to_string();
        let res = post_message(&self.config, &channel_id, Some(req)).await?;
        Ok(res)
    }

    pub async fn send_code(&self, channel_id: &Uuid, lang: &str, code: &str) -> Result<Message> {
        let message = formatdoc! {
            r#"
            ```{}
            {}
            ```
            "#,
            lang, code
        };
        self.send_message(channel_id, &message, false).await
    }

    pub async fn send_direct_message(
        &self,
        user_id: &Uuid,
        content: &str,
        embed: bool,
    ) -> Result<Message> {
        println!("send_dm: {}", user_id);
        let req = PostMessageRequest {
            content: content.to_string(),
            embed: Some(embed),
        };
        let user_id = user_id.to_string();
        let res = post_direct_message(&self.config, &user_id, Some(req)).await?;
        Ok(res)
    }

    pub async fn send_code_dm(&self, user_id: &Uuid, lang: &str, code: &str) -> Result<Message> {
        let message = formatdoc! {
            r#"
            ```{}
            {}
            ```
            "#,
            lang, code
        };
        self.send_direct_message(user_id, &message, false).await
    }

    pub async fn get_group_members(&self, group_id: &Uuid) -> Result<Vec<UserGroupMember>> {
        println!("get_group_members: {}", group_id);
        let group_id = group_id.to_string();
        let res = get_user_group_members(&self.config, &group_id).await?;
        Ok(res)
    }

    pub async fn get_user(&self, user_id: &Uuid) -> Result<UserDetail> {
        println!("get_user: {}", user_id);
        let user_id = user_id.to_string();
        let res = get_user(&self.config, &user_id).await?;
        Ok(res)
    }

    #[async_recursion]
    pub async fn get_channel_path(&self, channel_id: &Uuid) -> Result<String> {
        println!("get_channel_path: {}", channel_id);
        let channel_id = channel_id.to_string();
        let channel = get_channel(&self.config, &channel_id).await?;
        match channel.parent_id {
            Some(pid) => {
                let ppath = self.get_channel_path(&pid).await?;
                Ok(format!("#{}/{}", ppath, channel.name))
            }
            None => Ok(format!("#{}", channel.name)),
        }
    }
}
