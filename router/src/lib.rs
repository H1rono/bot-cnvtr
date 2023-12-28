use std::sync::Arc;

use axum::{
    extract::State,
    routing::{get, post},
    Router,
};
use tokio::sync::Mutex;

use traq_bot_http::RequestParser;

use domain::{Repository, TraqClient};
use usecases::Bot;
use wh_handler::WebhookHandler;

mod bot;
mod config;
mod error;
mod wh;

pub use config::Config;
use error::{Error, Result};

struct AppState<Repo, C, WH, E1, E2>
where
    Repo: Repository<Error = E1>,
    C: TraqClient<Error = E2>,
    WH: WebhookHandler,
    usecases::Error: From<E1> + From<E2>,
{
    pub repo: Arc<Mutex<Repo>>,
    pub client: Arc<Mutex<C>>,
    pub wh: WH,
    pub parser: RequestParser,
    pub bot: Bot,
}

impl<Repo, C, WH, E1, E2> Clone for AppState<Repo, C, WH, E1, E2>
where
    Repo: Repository<Error = E1>,
    C: TraqClient<Error = E2>,
    WH: WebhookHandler,
    usecases::Error: From<E1> + From<E2>,
{
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            repo: self.repo.clone(),
            wh: self.wh.clone(),
            parser: self.parser.clone(),
            bot: self.bot.clone(),
        }
    }
}

impl<Repo, C, WH, E1, E2> AppState<Repo, C, WH, E1, E2>
where
    Repo: Repository<Error = E1>,
    C: TraqClient<Error = E2>,
    WH: WebhookHandler,
    usecases::Error: From<E1> + From<E2>,
{
    pub fn new(client: C, repo: Repo, wh: WH, parser: RequestParser, bot: Bot) -> Self {
        Self {
            client: Arc::new(Mutex::new(client)),
            repo: Arc::new(Mutex::new(repo)),
            wh,
            parser,
            bot,
        }
    }
}

impl<Repo, C, WH, E1, E2> AsRef<AppState<Repo, C, WH, E1, E2>>
    for State<AppState<Repo, C, WH, E1, E2>>
where
    Repo: Repository<Error = E1>,
    C: TraqClient<Error = E2>,
    WH: WebhookHandler,
    usecases::Error: From<E1> + From<E2>,
{
    fn as_ref(&self) -> &AppState<Repo, C, WH, E1, E2> {
        &self.0
    }
}

pub fn make_router<Repo, C, WH, E1, E2>(
    config: Config,
    client: C,
    wh: WH,
    repo: Repo,
    bot: Bot,
) -> Router
where
    Repo: Repository<Error = E1>,
    C: TraqClient<Error = E2>,
    WH: WebhookHandler,
    E1: Send + Sync + 'static,
    E2: Send + Sync + 'static,
    usecases::Error: From<E1> + From<E2>,
{
    let parser = config.into();
    let state = AppState::new(client, repo, wh, parser, bot);
    Router::new()
        .route("/bot", post(bot::event::<Repo, C, WH, E1, E2>))
        .route("/wh/:id", get(wh::get_wh::<Repo, C, WH, E1, E2>))
        .route("/wh/:id/github", post(wh::wh_github::<Repo, C, WH, E1, E2>))
        .route("/wh/:id/gitea", post(wh::wh_gitea::<Repo, C, WH, E1, E2>))
        .route(
            "/wh/:id/clickup",
            post(wh::wh_clickup::<Repo, C, WH, E1, E2>),
        )
        .with_state(state)
}
