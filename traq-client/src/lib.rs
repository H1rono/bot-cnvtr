use async_trait::async_trait;

use traq::models::{Message, UserDetail, UserGroupMember};
use uuid::Uuid;

mod client_impl;
pub mod config;
pub mod error;

pub use client_impl::ClientImpl;
pub use config::Config;
pub use error::{Error, Result};

#[async_trait]
pub trait Client: Send + Sync + 'static {
    async fn send_message(&self, channel_id: &Uuid, content: &str, embed: bool) -> Result<Message>;

    async fn send_code(&self, channel_id: &Uuid, lang: &str, code: &str) -> Result<Message>;

    async fn send_direct_message(
        &self,
        user_id: &Uuid,
        content: &str,
        embed: bool,
    ) -> Result<Message>;

    async fn send_code_dm(&self, user_id: &Uuid, lang: &str, code: &str) -> Result<Message>;

    async fn get_group_members(&self, group_id: &Uuid) -> Result<Vec<UserGroupMember>>;

    async fn get_user(&self, user_id: &Uuid) -> Result<UserDetail>;

    async fn get_channel_path(&self, channel_id: &Uuid) -> Result<String>;

    async fn add_message_stamp(&self, message_id: &Uuid, stamp_id: &Uuid, count: i32)
        -> Result<()>;
}
