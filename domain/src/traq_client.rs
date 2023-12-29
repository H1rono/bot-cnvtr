use std::future::Future;

use uuid::Uuid;

use crate::group::Group;
use crate::user::User;

pub trait TraqClient: Send + Sync + 'static {
    type Error: Send + Sync + 'static;

    fn send_message(
        &self,
        channel_id: &Uuid,
        content: &str,
        embed: bool,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;

    fn send_code(
        &self,
        channel_id: &Uuid,
        lang: &str,
        code: &str,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;

    fn send_direct_message(
        &self,
        user_id: &Uuid,
        content: &str,
        embed: bool,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;

    fn send_code_dm(
        &self,
        user_id: &Uuid,
        lang: &str,
        code: &str,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;

    fn get_group(&self, group_id: &Uuid)
        -> impl Future<Output = Result<Group, Self::Error>> + Send;

    fn get_user(&self, user_id: &Uuid) -> impl Future<Output = Result<User, Self::Error>> + Send;

    fn get_channel_path(
        &self,
        channel_id: &Uuid,
    ) -> impl Future<Output = Result<String, Self::Error>> + Send;

    fn add_message_stamp(
        &self,
        message_id: &Uuid,
        stamp_id: &Uuid,
        count: i32,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;
}
