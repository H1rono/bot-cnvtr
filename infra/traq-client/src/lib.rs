use std::vec;

use anyhow::Context;
use itertools::Itertools;
use traq::apis::configuration::Configuration;

use domain::{ChannelId, Failure, Group, GroupId, MessageId, StampId, TraqClient, User, UserId};

#[must_use]
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
}

impl TraqClient for ClientImpl {
    #[tracing::instrument(skip_all)]
    async fn send_message(
        &self,
        channel_id: &ChannelId,
        content: &str,
        embed: bool,
    ) -> Result<(), Failure> {
        use traq::apis::message_api::post_message;
        use traq::models::PostMessageRequest;

        tracing::debug!("send_message: channel_id={}", channel_id);
        let req = PostMessageRequest {
            content: content.to_string(),
            embed: Some(embed),
        };
        let channel_id = channel_id.to_string();
        post_message(&self.config, &channel_id, Some(req))
            .await
            .context("Failed to post message to traQ")?;
        Ok(())
    }

    #[tracing::instrument(skip_all)]
    async fn send_direct_message(
        &self,
        user_id: &UserId,
        content: &str,
        embed: bool,
    ) -> Result<(), Failure> {
        use traq::apis::user_api::post_direct_message;
        use traq::models::PostMessageRequest;

        tracing::debug!("send_dm: user_id={}", user_id);
        let req = PostMessageRequest {
            content: content.to_string(),
            embed: Some(embed),
        };
        let user_id = user_id.to_string();
        post_direct_message(&self.config, &user_id, Some(req))
            .await
            .context("Failed to post direct message to traQ")?;
        Ok(())
    }

    #[tracing::instrument(skip_all)]
    async fn get_group(&self, group_id: &GroupId) -> Result<Group, Failure> {
        use traq::apis::group_api::get_user_group;

        tracing::debug!("get_group: group_id={}", group_id);
        let gid = group_id.to_string();
        let g = get_user_group(&self.config, &gid)
            .await
            .context("Failed to an user group from traQ")?;
        let mut members = vec![];
        for member in g.members {
            let member_id = member.id.into();
            let user = self.get_user(&member_id).await?;
            members.push(user);
        }
        let group = Group {
            id: *group_id,
            name: g.name.into(),
            members,
        };
        Ok(group)
    }

    #[tracing::instrument(skip_all)]
    async fn get_user(&self, user_id: &UserId) -> Result<User, Failure> {
        use traq::apis::user_api::get_user;

        tracing::debug!("get_user: user_id={}", user_id);
        let uid = user_id.to_string();
        let u = get_user(&self.config, &uid)
            .await
            .context("Failed to get an user from traQ")?;
        let user = User {
            id: u.id.into(),
            name: u.name.into(),
        };
        Ok(user)
    }

    #[tracing::instrument(skip_all)]
    async fn get_channel_path(&self, channel_id: &ChannelId) -> Result<String, Failure> {
        use traq::apis::channel_api::get_channel;

        tracing::debug!("get_channel_path: channel_id={}", channel_id);
        let mut channel_names: Vec<String> = vec![];
        let mut channel_id = Some(*channel_id);
        while let Some(id) = channel_id {
            let channel = get_channel(&self.config, &id.to_string())
                .await
                .context("Failed to get channel from traQ")?;
            channel_names.push(channel.name);
            channel_id = channel.parent_id.map(ChannelId::from);
        }
        Ok(format!("#{}", channel_names.into_iter().rev().join("/")))
    }

    #[tracing::instrument(skip_all)]
    async fn add_message_stamp(
        &self,
        message_id: &MessageId,
        stamp_id: &StampId,
        count: i32,
    ) -> Result<(), Failure> {
        use traq::apis::stamp_api::add_message_stamp;
        use traq::models::PostMessageStampRequest;

        tracing::debug!(
            "add_message_stamp: message_id={}, stamp_id={}",
            message_id,
            stamp_id
        );
        let req = PostMessageStampRequest { count };
        let message_id = message_id.to_string();
        let stamp_id = stamp_id.to_string();
        add_message_stamp(&self.config, &message_id, &stamp_id, Some(req))
            .await
            .context("Failed to add a message stamp to traQ")?;
        Ok(())
    }
}
