use traq_bot_http::Event;

use domain::Infra;
use usecases::Bot;

#[derive(Clone)]
pub struct BotWrapper<B>(pub B);

impl<I, B> Bot<I> for BotWrapper<B>
where
    I: Infra,
    B: Bot<I, Error = I::Error>,
    usecases::Error: From<I::Error>,
{
    type Error = usecases::Error;

    async fn handle_event(&self, infra: &I, event: Event) -> Result<(), Self::Error> {
        Ok(self.0.handle_event(infra, event).await?)
    }
}
