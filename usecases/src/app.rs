use domain::Infra;

use crate::{Bot, WebhookHandler};

pub trait App<I: Infra>: Send + Sync + 'static {
    type Bot: Bot<I, Error = Self::Error>;
    type WebhookHandler: WebhookHandler<Error = Self::Error>;
    type Error: Send + Sync + 'static;

    fn bot(&self) -> &Self::Bot;
    fn webhook_handler(&self) -> &Self::WebhookHandler;
}
