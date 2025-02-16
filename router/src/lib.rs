use std::sync::Arc;

use axum::{
    routing::{get, post, post_service},
    Router,
};

use domain::Infra;
use usecases::{App, Bot, WebhookHandler};

mod error;
mod webhook;

trait AppState: Clone + Send + Sync + 'static {
    type Infra: Infra;
    type WebhookHandler: WebhookHandler<Self::Infra>;

    fn infra(&self) -> &Self::Infra;

    fn webhook_handler(&self) -> &Self::WebhookHandler;
}

#[must_use]
struct AppStateImpl<I, WH>
where
    I: Infra,
    WH: WebhookHandler<I>,
{
    pub infra: Arc<I>,
    pub webhook_handler: Arc<WH>,
}

impl<I, WH> AppState for AppStateImpl<I, WH>
where
    I: Infra,
    WH: WebhookHandler<I>,
{
    type Infra = I;
    type WebhookHandler = WH;

    fn infra(&self) -> &Self::Infra {
        &self.infra
    }

    fn webhook_handler(&self) -> &Self::WebhookHandler {
        &self.webhook_handler
    }
}

impl<I, WH> Clone for AppStateImpl<I, WH>
where
    I: Infra,
    WH: WebhookHandler<I>,
{
    fn clone(&self) -> Self {
        Self {
            infra: self.infra.clone(),
            webhook_handler: self.webhook_handler.clone(),
        }
    }
}

impl<I, WH> AppStateImpl<I, WH>
where
    I: Infra,
    WH: WebhookHandler<I>,
{
    pub fn new(infra: Arc<I>, webhook_handler: Arc<WH>) -> Self {
        Self {
            infra,
            webhook_handler,
        }
    }
}

pub fn make_router<I, A>(infra: Arc<I>, app: A) -> Router
where
    I: Infra,
    A: App<I>,
{
    use webhook::{get_wh, wh_clickup, wh_gitea, wh_github};

    let (bot, webhook_handler) = app.split();
    let state = AppStateImpl::new(Arc::clone(&infra), Arc::new(webhook_handler));
    let bot_service = bot.build_service::<axum::body::Body>(infra);
    let bot_service = post_service(bot_service).handle_error(|err| async move {
        use axum::response::IntoResponse;
        crate::error::Error::from(err).into_response()
    });
    Router::new()
        .route("/bot", bot_service)
        .route(
            "/wh/{id}",
            get(get_wh::<AppStateImpl<I, A::WebhookHandler>>),
        )
        .route(
            "/wh/{id}/github",
            post(wh_github::<AppStateImpl<I, A::WebhookHandler>>),
        )
        .route(
            "/wh/{id}/gitea",
            post(wh_gitea::<AppStateImpl<I, A::WebhookHandler>>),
        )
        .route(
            "/wh/{id}/clickup",
            post(wh_clickup::<AppStateImpl<I, A::WebhookHandler>>),
        )
        .with_state(state)
}

pub use axum::serve;
