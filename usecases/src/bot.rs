use std::future::Future;

use traq_bot_http::Event;

use domain::Infra;

use crate::error::Result;

pub trait Bot<I: Infra>: Send + Sync + 'static {
    type Error: Send + Sync + 'static;

    fn handle_event(
        &self,
        infra: &I,
        event: Event,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;
}
