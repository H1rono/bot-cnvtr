use std::vec;

use async_trait::async_trait;
use indoc::formatdoc;
use itertools::Itertools;

use traq::apis::channel_api::get_channel;
use traq::apis::configuration::Configuration;
use traq::apis::group_api::get_user_group_members;
use traq::apis::message_api::post_message;
use traq::apis::stamp_api::add_message_stamp;
use traq::apis::user_api::{get_user, post_direct_message};
use traq::models::{
    Message, PostMessageRequest, PostMessageStampRequest, UserDetail, UserGroupMember,
};
use uuid::Uuid;

use crate::{Config, Result};

#[derive(Debug, Clone)]
pub struct ClientImpl {
    pub config: Configuration,
}

impl ClientImpl {
    pub fn new(bot_access_token: &str) -> Self {
        let config = Configuration {
            bearer_access_token: Some(bot_access_token.to_string()),
            ..Default::default()
        };
        Self { config }
    }

    pub fn from_config(config: Config) -> Self {
        let config = Configuration {
            bearer_access_token: Some(config.bot_access_token),
            ..Default::default()
        };
        Self { config }
    }
}

#[async_trait]
impl crate::Client for ClientImpl {
    async fn send_message(&self, channel_id: &Uuid, content: &str, embed: bool) -> Result<Message> {
        println!("send_message: {}", channel_id);
        let req = PostMessageRequest {
            content: content.to_string(),
            embed: Some(embed),
        };
        let channel_id = channel_id.to_string();
        let res = post_message(&self.config, &channel_id, Some(req)).await?;
        Ok(res)
    }

    async fn send_code(&self, channel_id: &Uuid, lang: &str, code: &str) -> Result<Message> {
        let message = formatdoc! {
            r#"
            ```{}
            {}
            ```
            "#,
            lang, code
        };
        self.send_message(channel_id, message.trim(), false).await
    }

    async fn send_direct_message(
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

    async fn send_code_dm(&self, user_id: &Uuid, lang: &str, code: &str) -> Result<Message> {
        let message = formatdoc! {
            r#"
            ```{}
            {}
            ```
            "#,
            lang, code
        };
        self.send_direct_message(user_id, message.trim(), false)
            .await
    }

    async fn get_group_members(&self, group_id: &Uuid) -> Result<Vec<UserGroupMember>> {
        println!("get_group_members: {}", group_id);
        let group_id = group_id.to_string();
        let res = get_user_group_members(&self.config, &group_id).await?;
        Ok(res)
    }

    async fn get_user(&self, user_id: &Uuid) -> Result<UserDetail> {
        println!("get_user: {}", user_id);
        let user_id = user_id.to_string();
        let res = get_user(&self.config, &user_id).await?;
        Ok(res)
    }

    async fn get_channel_path(&self, channel_id: &Uuid) -> Result<String> {
        println!("get_channel_path: {}", channel_id);
        let mut channel_names: Vec<String> = vec![];
        let mut channel_id = Some(*channel_id);
        while let Some(id) = channel_id {
            let channel = get_channel(&self.config, &id.to_string()).await?;
            channel_names.push(channel.name);
            channel_id = channel.parent_id;
        }
        Ok(format!("#{}", channel_names.into_iter().rev().join("/")))
    }

    async fn add_message_stamp(
        &self,
        message_id: &Uuid,
        stamp_id: &Uuid,
        count: i32,
    ) -> Result<()> {
        println!("add_message_stamp: {}, {}", message_id, stamp_id);
        let req = PostMessageStampRequest { count };
        let message_id = message_id.to_string();
        let stamp_id = stamp_id.to_string();
        add_message_stamp(&self.config, &message_id, &stamp_id, Some(req)).await?;
        Ok(())
    }
}
