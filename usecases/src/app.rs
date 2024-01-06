use domain::Infra;

use crate::{Bot, WebhookHandler};

pub trait App<I: Infra>: Send + Sync + 'static {
    type Error: Into<domain::Error> + Send + Sync + 'static;
    type Bot: Bot<I, Error = Self::Error>;
    type WebhookHandler: WebhookHandler<I, Error = Self::Error>;

    fn bot(&self) -> &Self::Bot;
    fn webhook_handler(&self) -> &Self::WebhookHandler;
}
