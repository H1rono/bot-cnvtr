use std::{error, fmt::Display};

use traq::apis::configuration::Configuration;
use traq_bot_http::Event;

use super::{config::BotConfig, Database};

#[derive(Debug, Clone)]
pub struct Bot {
    pub id: String,
    pub user_id: String,
    pub config: Configuration,
}

impl Bot {
    pub fn new(id: &str, user_id: &str, access_token: &str) -> Self {
        let id = id.to_string();
        let user_id = user_id.to_string();
        let config = Configuration {
            bearer_access_token: Some(access_token.to_string()),
            ..Default::default()
        };
        Self {
            id,
            user_id,
            config,
        }
    }

    pub fn from_config(bot_config: BotConfig) -> Self {
        let BotConfig {
            bot_id,
            bot_user_id,
            bot_access_token,
            ..
        } = bot_config;
        let config = Configuration {
            bearer_access_token: Some(bot_access_token),
            ..Default::default()
        };
        Self {
            id: bot_id,
            user_id: bot_user_id,
            config,
        }
    }

    pub async fn handle_event(&self, db: &Database, event: Event) -> Result<(), Error> {
        use Event::*;
        match event {
            Joined(payload) => {
                println!("チャンネル {} に参加しました。", payload.channel.name);
                Ok(())
            }
            Left(payload) => {
                println!("チャンネル {} から退出しました。", payload.channel.name);
                Ok(())
            }
            MessageCreated(payload) => {
                print!(
                    "{}さんがメッセージを投稿しました。\n内容: {}\n",
                    payload.message.user.display_name, payload.message.text
                );
                Ok(())
            }
            _ => Ok(()),
        }?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Error;

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "bot::Error")
    }
}

impl error::Error for Error {}
