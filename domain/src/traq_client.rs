use std::future::Future;

use crate::group::{Group, GroupId};
use crate::user::{User, UserId};
use crate::webhook::ChannelId;

crate::macros::newtype_id! {Message}
crate::macros::newtype_id! {Stamp}

pub trait TraqClient: Send + Sync + 'static {
    type Error: Into<crate::error::Error> + Send + Sync + 'static;

    fn send_message(
        &self,
        channel_id: &ChannelId,
        content: &str,
        embed: bool,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;

    fn send_code(
        &self,
        channel_id: &ChannelId,
        lang: &str,
        code: &str,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send {
        async move {
            let message = indoc::formatdoc! {
                r#"
                    ```{}
                    {}
                    ```
                "#,
                lang, code
            };
            self.send_message(channel_id, &message, false).await
        }
    }

    fn send_direct_message(
        &self,
        user_id: &UserId,
        content: &str,
        embed: bool,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;

    fn send_code_dm(
        &self,
        user_id: &UserId,
        lang: &str,
        code: &str,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send {
        async move {
            let message = indoc::formatdoc! {
                r#"
                    ```{}
                    {}
                    ```
                "#,
                lang, code
            };
            self.send_direct_message(user_id, &message, false).await
        }
    }

    fn get_group(
        &self,
        group_id: &GroupId,
    ) -> impl Future<Output = Result<Group, Self::Error>> + Send;

    fn get_user(&self, user_id: &UserId) -> impl Future<Output = Result<User, Self::Error>> + Send;

    fn get_channel_path(
        &self,
        channel_id: &ChannelId,
    ) -> impl Future<Output = Result<String, Self::Error>> + Send;

    fn add_message_stamp(
        &self,
        message_id: &MessageId,
        stamp_id: &StampId,
        count: i32,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;
}
