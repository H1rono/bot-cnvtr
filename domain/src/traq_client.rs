use async_trait::async_trait;
use uuid::Uuid;

use crate::group::Group;
use crate::user::User;

#[async_trait]
pub trait TraqClient: Send + Sync + 'static {
    type Error: Send + Sync + 'static;

    async fn send_message(
        &self,
        channel_id: &Uuid,
        content: &str,
        embed: bool,
    ) -> Result<(), Self::Error>;

    async fn send_code(&self, channel_id: &Uuid, lang: &str, code: &str)
        -> Result<(), Self::Error>;

    async fn send_direct_message(
        &self,
        user_id: &Uuid,
        content: &str,
        embed: bool,
    ) -> Result<(), Self::Error>;

    async fn send_code_dm(&self, user_id: &Uuid, lang: &str, code: &str)
        -> Result<(), Self::Error>;

    async fn get_group(&self, group_id: &Uuid) -> Result<Group, Self::Error>;

    async fn get_user(&self, user_id: &Uuid) -> Result<User, Self::Error>;

    async fn get_channel_path(&self, channel_id: &Uuid) -> Result<String, Self::Error>;

    async fn add_message_stamp(
        &self,
        message_id: &Uuid,
        stamp_id: &Uuid,
        count: i32,
    ) -> Result<(), Self::Error>;
}
