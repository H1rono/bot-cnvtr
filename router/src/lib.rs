use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use traq_bot_http::RequestParser;

use domain::Infra;
use usecases::App;

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

struct AppStateImpl<I, A>
where
    I: Infra<Error = usecases::Error>,
    A: App<I, Error = usecases::Error>,
{
    pub infra: Arc<I>,
    pub app: Arc<A>,
    pub parser: RequestParser,
}

impl<I, A> AppState for AppStateImpl<I, A>
where
    I: Infra<Error = usecases::Error>,
    A: App<I, Error = usecases::Error>,
{
    type Infra = I;
    type App = A;
    type Error = usecases::Error;

    fn infra(&self) -> &Self::Infra {
        &self.infra
    }

    fn app(&self) -> &Self::App {
        &self.app
    }
}

impl<I, A> Clone for AppStateImpl<I, A>
where
    I: Infra<Error = usecases::Error>,
    A: App<I, Error = usecases::Error>,
{
    fn clone(&self) -> Self {
        Self {
            infra: self.infra.clone(),
            app: self.app.clone(),
            parser: self.parser.clone(),
        }
    }
}

impl<I, A> AppStateImpl<I, A>
where
    I: Infra<Error = usecases::Error>,
    A: App<I, Error = usecases::Error>,
{
    pub fn new(infra: I, app: A, parser: RequestParser) -> Self {
        Self {
            infra: Arc::new(infra),
            app: Arc::new(app),
            parser,
        }
    }
}

pub fn make_router<I, A>(config: Config, infra: I, app: A) -> Router
where
    I: Infra<Error = usecases::Error>,
    A: App<I, Error = usecases::Error>,
{
    let parser = config.into();
    let state = AppStateImpl::new(infra, app, parser);
    Router::new()
        .route("/bot", post(bot::event::<I, A>))
        .route("/wh/:id", get(wh::get_wh::<I, A>))
        .route("/wh/:id/github", post(wh::wh_github::<I, A>))
        .route("/wh/:id/gitea", post(wh::wh_gitea::<I, A>))
        .route("/wh/:id/clickup", post(wh::wh_clickup::<I, A>))
        .with_state(state)
}
