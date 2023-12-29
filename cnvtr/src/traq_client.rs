use domain::TraqClient;

pub struct TraqClientWrapper<C: TraqClient>(pub C);

impl<C> TraqClient for TraqClientWrapper<C>
where
    C: TraqClient,
    usecases::Error: From<C::Error>,
{
    type Error = usecases::Error;

    async fn send_message(
        &self,
        channel_id: &uuid::Uuid,
        content: &str,
        embed: bool,
    ) -> Result<(), Self::Error> {
        Ok(self.0.send_message(channel_id, content, embed).await?)
    }

    async fn send_code(
        &self,
        channel_id: &uuid::Uuid,
        lang: &str,
        code: &str,
    ) -> Result<(), Self::Error> {
        Ok(self.0.send_code(channel_id, lang, code).await?)
    }

    async fn send_direct_message(
        &self,
        user_id: &uuid::Uuid,
        content: &str,
        embed: bool,
    ) -> Result<(), Self::Error> {
        Ok(self.0.send_direct_message(user_id, content, embed).await?)
    }

    async fn send_code_dm(
        &self,
        user_id: &uuid::Uuid,
        lang: &str,
        code: &str,
    ) -> Result<(), Self::Error> {
        Ok(self.0.send_code_dm(user_id, lang, code).await?)
    }

    async fn get_group(&self, group_id: &uuid::Uuid) -> Result<domain::Group, Self::Error> {
        Ok(self.0.get_group(group_id).await?)
    }

    async fn get_user(&self, user_id: &uuid::Uuid) -> Result<domain::User, Self::Error> {
        Ok(self.0.get_user(user_id).await?)
    }

    async fn get_channel_path(&self, channel_id: &uuid::Uuid) -> Result<String, Self::Error> {
        Ok(self.0.get_channel_path(channel_id).await?)
    }

    async fn add_message_stamp(
        &self,
        message_id: &uuid::Uuid,
        stamp_id: &uuid::Uuid,
        count: i32,
    ) -> Result<(), Self::Error> {
        Ok(self
            .0
            .add_message_stamp(message_id, stamp_id, count)
            .await?)
    }
}
