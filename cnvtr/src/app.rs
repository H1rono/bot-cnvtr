use domain::Infra;
use usecases::{App, Bot, WebhookHandler};

pub struct AppImpl<B, W>(pub B, pub W);

impl<I, B, W> App<I> for AppImpl<B, W>
where
    I: Infra<Error = usecases::Error>,
    B: Bot<I, Error = usecases::Error>,
    W: WebhookHandler<Error = usecases::Error>,
{
    type Error = usecases::Error;
    type Bot = B;
    type WebhookHandler = W;

    fn bot(&self) -> &Self::Bot {
        &self.0
    }

    fn webhook_handler(&self) -> &Self::WebhookHandler {
        &self.1
    }
}
