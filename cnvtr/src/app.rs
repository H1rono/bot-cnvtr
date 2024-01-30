use std::marker::PhantomData;

use domain::Infra;
use usecases::{App, Bot, WebhookHandler};

use crate::bot::BotWrapper;
use crate::wh_handler::WHandlerWrapper;

pub struct AppImpl<B, W, I = ()>(pub B, pub W, PhantomData<I>);

impl<B, W, I> AppImpl<B, W, I> {
    pub fn new(b: B, w: W) -> Self {
        AppImpl(b, w, PhantomData)
    }
}

impl<I, B, W> AppImpl<B, W, I>
where
    I: Infra,
    B: Bot<I>,
    W: WebhookHandler<I>,
    domain::Error: From<I::Error> + From<B::Error> + From<W::Error>,
{
    pub fn new_wrapped(b: B, w: W) -> AppImpl<BotWrapper<I, B>, WHandlerWrapper<I, W>> {
        let b = BotWrapper::new(b);
        let w = WHandlerWrapper::new(w);
        AppImpl(b, w, PhantomData)
    }
}

impl<I, B, W> App<I> for AppImpl<B, W>
where
    I: Infra<Error = domain::Error>,
    B: Bot<I, Error = domain::Error>,
    W: WebhookHandler<I, Error = domain::Error>,
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
