use domain::Infra;
use usecases::{App, Bot, WebhookHandler};

pub struct AppImpl<B, W>(pub B, pub W);

impl<I, B, W> App<I> for AppImpl<B, W>
where
    I: Infra<Error = domain::Error>,
    B: Bot<I, Error = domain::Error>,
    W: WebhookHandler<Error = domain::Error>,
{
    type Error = domain::Error;
    type Bot = B;
    type WebhookHandler = W;

    fn bot(&self) -> &Self::Bot {
        &self.0
    }

    fn webhook_handler(&self) -> &Self::WebhookHandler {
        &self.1
    }
}
