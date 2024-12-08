use std::sync::Arc;

use http::Request;
use traq_bot_http::RequestParser;

use domain::Infra;
use usecases::Bot;

mod builder;
pub(crate) mod cli;
mod error;
mod messages;
mod state;
mod system;

pub use error::{Error, Result};

static HELP_TEMPLATE: &str = include_str!("help.md");

#[must_use]
#[derive(Debug, Clone)]
pub struct BotImpl {
    parser: RequestParser,
    inner: BotImplInner,
}

#[must_use]
#[expect(unused)]
#[derive(Debug, Clone)]
struct BotImplInner {
    pub name: String,
    pub id: String,
    pub user_id: String,
}

impl<I: Infra> Bot<I> for BotImpl {
    fn build_service<B>(
        self,
        infra: Arc<I>,
    ) -> tower::util::BoxCloneService<http::Request<B>, http::Response<String>, domain::Error>
    where
        B: http_body::Body + Send + 'static,
        B::Data: Send + 'static,
        B::Error: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
    {
        use tower::{service_fn, ServiceExt};

        let Self { parser, inner } = self;
        let state = State { infra, bot: inner };
        let handler = parser
            .into_handler()
            .on_joined(service_fn(State::on_joined))
            .on_left(service_fn(State::on_left))
            .on_message_created(service_fn(State::on_message_created))
            .on_direct_message_created(service_fn(State::on_direct_message_created))
            .with_state(Arc::new(state))
            .map_request(|r: Request<B>| r)
            .map_err(Error::TraqBot)
            .map_err(domain::Error::from);
        handler.boxed_clone()
    }
}

#[must_use]
#[derive(Clone)]
pub struct State<I> {
    infra: Arc<I>,
    bot: BotImplInner,
}

#[must_use]
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Builder {
    verification_token: Option<String>,
    name: Option<String>,
    id: Option<String>,
    user_id: Option<String>,
}
