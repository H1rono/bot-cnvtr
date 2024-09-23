use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use traq_bot_http::RequestParser;

use domain::Infra;
use usecases::App;

mod bot;
mod error;
mod webhook;

trait AppState: Clone + Send + Sync + 'static {
    type Infra: Infra;
    type App: App<Self::Infra, Error = Self::Error>;
    type Error: Send + Sync + 'static;

    fn infra(&self) -> &Self::Infra;

    fn app(&self) -> &Self::App;

    fn parser(&self) -> &RequestParser;
}

#[must_use]
struct AppStateImpl<I, A>
where
    I: Infra,
    A: App<I, Error = domain::Error>,
{
    pub infra: Arc<I>,
    pub app: Arc<A>,
    pub parser: RequestParser,
}

impl<I, A> AppState for AppStateImpl<I, A>
where
    I: Infra,
    A: App<I, Error = domain::Error>,
{
    type Infra = I;
    type App = A;
    type Error = domain::Error;

    fn infra(&self) -> &Self::Infra {
        &self.infra
    }

    fn app(&self) -> &Self::App {
        &self.app
    }

    fn parser(&self) -> &RequestParser {
        &self.parser
    }
}

impl<I, A> Clone for AppStateImpl<I, A>
where
    I: Infra,
    A: App<I, Error = domain::Error>,
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
    I: Infra,
    A: App<I, Error = domain::Error>,
{
    pub fn new(infra: Arc<I>, app: Arc<A>, parser: RequestParser) -> Self {
        Self { infra, app, parser }
    }
}

pub fn make_router<I, A>(verification_token: &str, infra: Arc<I>, app: Arc<A>) -> Router
where
    I: Infra,
    A: App<I, Error = domain::Error>,
{
    use webhook::{get_wh, wh_clickup, wh_gitea, wh_github};

    let parser = RequestParser::new(verification_token);
    let state = AppStateImpl::new(infra, app, parser);
    Router::new()
        .route("/bot", post(bot::event::<AppStateImpl<I, A>>))
        .route("/wh/:id", get(get_wh::<AppStateImpl<I, A>>))
        .route("/wh/:id/github", post(wh_github::<AppStateImpl<I, A>>))
        .route("/wh/:id/gitea", post(wh_gitea::<AppStateImpl<I, A>>))
        .route("/wh/:id/clickup", post(wh_clickup::<AppStateImpl<I, A>>))
        .with_state(state)
}

pub use axum::serve;
