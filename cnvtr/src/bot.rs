use std::marker::PhantomData;

use traq_bot_http::Event;

use domain::Infra;
use usecases::Bot;

#[derive(Clone)]
pub struct BotWrapper<I: Infra, B: Bot<I>>(pub B, PhantomData<I>);

impl<I: Infra, B: Bot<I>> BotWrapper<I, B> {
    pub fn new(bot: B) -> Self {
        Self(bot, PhantomData)
    }
}

impl<I, B> Bot<I> for BotWrapper<I, B>
where
    I: Infra,
    B: Bot<I, Error = I::Error>,
    domain::Error: From<I::Error>,
{
    type Error = domain::Error;

    async fn handle_event(&self, infra: &I, event: Event) -> Result<(), Self::Error> {
        Ok(self.0.handle_event(infra, event).await?)
    }
}
