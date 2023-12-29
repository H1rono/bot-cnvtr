use std::sync::Arc;

use axum::{
    extract::State,
    routing::{get, post},
    Router,
};

use traq_bot_http::RequestParser;

use domain::{Infra, Repository, TraqClient};
use usecases::BotImpl;
use usecases::{App, WebhookHandler};

mod bot;
mod config;
mod error;
mod wh;

pub use config::Config;
use error::{Error, Result};

trait AppState {
    type Infra: Infra<Error = Self::Error>;
    type App: App<Self::Infra, Error = Self::Error>;
    type Error: Send + Sync + 'static;

    fn infra(&self) -> &Self::Infra;
    fn repo(&self) -> &<Self::Infra as Infra>::Repo {
        self.infra().repo()
    }
    fn traq_client(&self) -> &<Self::Infra as Infra>::TClient {
        self.infra().traq_client()
    }

    fn app(&self) -> &Self::App;
    fn bot(&self) -> &<Self::App as App<Self::Infra>>::Bot {
        self.app().bot()
    }
    fn webhook_handler(&self) -> &<Self::App as App<Self::Infra>>::WebhookHandler {
        self.app().webhook_handler()
    }
}

struct AppStateImpl<I, WH, E1, E2, E3>
where
    I: Infra,
    I::Repo: Repository<Error = E1>,
    I::TClient: TraqClient<Error = E2>,
    WH: WebhookHandler<Error = E3>,
    usecases::Error: From<E1> + From<E2> + From<E3>,
{
    pub infra: Arc<I>,
    pub wh: WH,
    pub parser: RequestParser,
    pub bot: BotImpl,
}

impl<I, WH, E1, E2, E3> Clone for AppStateImpl<I, WH, E1, E2, E3>
where
    I: Infra,
    I::Repo: Repository<Error = E1>,
    I::TClient: TraqClient<Error = E2>,
    WH: WebhookHandler<Error = E3>,
    usecases::Error: From<E1> + From<E2> + From<E3>,
{
    fn clone(&self) -> Self {
        Self {
            infra: self.infra.clone(),
            wh: self.wh.clone(),
            parser: self.parser.clone(),
            bot: self.bot.clone(),
        }
    }
}

impl<I, WH, E1, E2, E3> AppStateImpl<I, WH, E1, E2, E3>
where
    I: Infra,
    I::Repo: Repository<Error = E1>,
    I::TClient: TraqClient<Error = E2>,
    WH: WebhookHandler<Error = E3>,
    usecases::Error: From<E1> + From<E2> + From<E3>,
{
    pub fn new(infra: I, wh: WH, parser: RequestParser, bot: BotImpl) -> Self {
        Self {
            infra: Arc::new(infra),
            wh,
            parser,
            bot,
        }
    }
}

impl<I, WH, E1, E2, E3> AsRef<AppStateImpl<I, WH, E1, E2, E3>>
    for State<AppStateImpl<I, WH, E1, E2, E3>>
where
    I: Infra,
    I::Repo: Repository<Error = E1>,
    I::TClient: TraqClient<Error = E2>,
    WH: WebhookHandler<Error = E3>,
    usecases::Error: From<E1> + From<E2> + From<E3>,
{
    fn as_ref(&self) -> &AppStateImpl<I, WH, E1, E2, E3> {
        &self.0
    }
}

pub fn make_router<I, WH, E1, E2, E3>(config: Config, infra: I, wh: WH, bot: BotImpl) -> Router
where
    I: Infra,
    I::Repo: Repository<Error = E1>,
    I::TClient: TraqClient<Error = E2>,
    WH: WebhookHandler<Error = E3>,
    usecases::Error: From<E1> + From<E2> + From<E3>,
    E1: Send + Sync + 'static,
    E2: Send + Sync + 'static,
    E3: Send + Sync + 'static,
{
    let parser = config.into();
    let state = AppStateImpl::new(infra, wh, parser, bot);
    Router::new()
        .route("/bot", post(bot::event::<I, WH, E1, E2, E3>))
        .route("/wh/:id", get(wh::get_wh::<I, WH, E1, E2, E3>))
        .route("/wh/:id/github", post(wh::wh_github::<I, WH, E1, E2, E3>))
        .route("/wh/:id/gitea", post(wh::wh_gitea::<I, WH, E1, E2, E3>))
        .route("/wh/:id/clickup", post(wh::wh_clickup::<I, WH, E1, E2, E3>))
        .with_state(state)
}
