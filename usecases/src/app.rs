use domain::Infra;

use crate::{Bot, WebhookHandler};

#[must_use]
pub trait App<I: Infra>: Send + Sync + 'static {
    type Bot: Bot<I>;
    type WebhookHandler: WebhookHandler<I>;

    fn bot(&self) -> &Self::Bot;
    fn webhook_handler(&self) -> &Self::WebhookHandler;
}
