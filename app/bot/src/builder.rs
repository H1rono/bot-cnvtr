use anyhow::Context;

use super::{BotImpl, BotImplInner, Builder};

impl BotImpl {
    pub fn builder() -> Builder {
        Builder::default()
    }
}

impl Builder {
    pub fn verification_token(self, value: &str) -> Self {
        let verification_token = Some(value.to_string());
        Self {
            verification_token,
            ..self
        }
    }

    pub fn name(self, value: &str) -> Self {
        let name = Some(value.to_string());
        Self { name, ..self }
    }

    pub fn id(self, value: &str) -> Self {
        let id = Some(value.to_string());
        Self { id, ..self }
    }

    pub fn user_id(self, value: &str) -> Self {
        let user_id = Some(value.to_string());
        Self { user_id, ..self }
    }

    pub fn build(self) -> anyhow::Result<BotImpl> {
        let verification_token = self
            .verification_token
            .context("bot verification_token not set")?;
        let name = self.name.context("bot name not set")?;
        let id = self.id.context("bot id not set")?;
        let user_id = self.user_id.context("bot user_id not set")?;
        let parser = traq_bot_http::RequestParser::new(&verification_token);
        let inner = BotImplInner { name, id, user_id };
        let bot = BotImpl { parser, inner };
        Ok(bot)
    }
}
