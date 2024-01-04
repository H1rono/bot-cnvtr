use domain::{ChannelId, GroupId, MessageId, StampId, TraqClient, UserId};

pub struct TraqClientWrapper<C: TraqClient>(pub C);

impl<C> TraqClient for TraqClientWrapper<C>
where
    C: TraqClient,
    domain::Error: From<C::Error>,
{
    type Error = domain::Error;

    async fn send_message(
        &self,
        channel_id: &ChannelId,
        content: &str,
        embed: bool,
    ) -> Result<(), Self::Error> {
        Ok(self.0.send_message(channel_id, content, embed).await?)
    }

    async fn send_code(
        &self,
        channel_id: &ChannelId,
        lang: &str,
        code: &str,
    ) -> Result<(), Self::Error> {
        Ok(self.0.send_code(channel_id, lang, code).await?)
    }

    async fn send_direct_message(
        &self,
        user_id: &UserId,
        content: &str,
        embed: bool,
    ) -> Result<(), Self::Error> {
        Ok(self.0.send_direct_message(user_id, content, embed).await?)
    }

    async fn send_code_dm(
        &self,
        user_id: &UserId,
        lang: &str,
        code: &str,
    ) -> Result<(), Self::Error> {
        Ok(self.0.send_code_dm(user_id, lang, code).await?)
    }

    async fn get_group(&self, group_id: &GroupId) -> Result<domain::Group, Self::Error> {
        Ok(self.0.get_group(group_id).await?)
    }

    async fn get_user(&self, user_id: &UserId) -> Result<domain::User, Self::Error> {
        Ok(self.0.get_user(user_id).await?)
    }

    async fn get_channel_path(&self, channel_id: &ChannelId) -> Result<String, Self::Error> {
        Ok(self.0.get_channel_path(channel_id).await?)
    }

    async fn add_message_stamp(
        &self,
        message_id: &MessageId,
        stamp_id: &StampId,
        count: i32,
    ) -> Result<(), Self::Error> {
        Ok(self
            .0
            .add_message_stamp(message_id, stamp_id, count)
            .await?)
    }
}
