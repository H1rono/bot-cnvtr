pub mod app;
pub mod infra;

use std::marker::PhantomData;

use domain::{EventSubscriber, Infra, Repository, TraqClient};
use usecases::{App, Bot, WebhookHandler};

use app::{BotWrapper, WHandlerWrapper};
use infra::{EventSubWrapper, RepoWrapper, TraqClientWrapper};

#[must_use]
pub struct InfraImpl<R, C, S>(pub R, pub C, pub S);

impl<R: Repository, C: TraqClient, S: EventSubscriber> InfraImpl<R, C, S> {
    pub fn new(repo: R, client: C, subscriber: S) -> Self {
        Self(repo, client, subscriber)
    }
}

impl<R: Repository, C: TraqClient, S: EventSubscriber>
    InfraImpl<RepoWrapper<R>, TraqClientWrapper<C>, EventSubWrapper<S>>
{
    pub fn new_wrapped(repo: R, client: C, subscriber: S) -> Self {
        let repo = RepoWrapper(repo);
        let client = TraqClientWrapper(client);
        let subscriber = EventSubWrapper(subscriber);
        Self(repo, client, subscriber)
    }
}

impl<R, C, S> Infra for InfraImpl<RepoWrapper<R>, TraqClientWrapper<C>, EventSubWrapper<S>>
where
    R: Repository,
    C: TraqClient,
    S: EventSubscriber + Clone,
{
    type Repo = RepoWrapper<R>;
    type TClient = TraqClientWrapper<C>;
    type ESub = EventSubWrapper<S>;

    fn repo(&self) -> &Self::Repo {
        &self.0
    }

    fn traq_client(&self) -> &Self::TClient {
        &self.1
    }

    fn event_subscriber(&self) -> &Self::ESub {
        &self.2
    }
}

#[must_use]
pub struct AppImpl<B, W, I = ()>(pub B, pub W, PhantomData<I>);

impl<B, W, I> AppImpl<B, W, I> {
    pub fn new(b: B, w: W) -> Self {
        AppImpl(b, w, PhantomData)
    }
}

impl<I, B, W> AppImpl<B, W, I>
where
    I: Infra,
    B: Bot<I>,
    W: WebhookHandler<I>,
{
    pub fn new_wrapped(b: B, w: W) -> AppImpl<BotWrapper<I, B>, WHandlerWrapper<I, W>> {
        let b = BotWrapper::new(b);
        let w = WHandlerWrapper::new(w);
        AppImpl(b, w, PhantomData)
    }
}

impl<I, B, W> App<I> for AppImpl<B, W>
where
    I: Infra,
    B: Bot<I>,
    W: WebhookHandler<I>,
{
    type Bot = B;
    type WebhookHandler = W;

    fn bot(&self) -> &Self::Bot {
        &self.0
    }

    fn webhook_handler(&self) -> &Self::WebhookHandler {
        &self.1
    }
}
