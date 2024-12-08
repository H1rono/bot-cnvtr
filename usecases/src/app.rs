use domain::Infra;

use crate::{Bot, WebhookHandler};

#[must_use]
pub trait App<I: Infra> {
    type Bot: Bot<I>;
    type WebhookHandler: WebhookHandler<I>;

    fn split(self) -> (Self::Bot, Self::WebhookHandler);
}

impl<I, B, WH> App<I> for (B, WH)
where
    I: Infra,
    B: Bot<I>,
    WH: WebhookHandler<I>,
{
    type Bot = B;
    type WebhookHandler = WH;

    fn split(self) -> (Self::Bot, Self::WebhookHandler) {
        self
    }
}
