use std::future::Future;

pub trait Bot {
    type Error: Send + Sync + 'static;

    fn handle_event<I: domain::Infra>(
        &self,
        infra: &I,
        event: traq_bot_http::Event,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;
}
