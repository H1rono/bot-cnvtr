mod app;
mod infra;

use std::marker::PhantomData;

use domain::{EventSubscriber, Infra, Repository, TraqClient};
use usecases::{App, Bot, WebhookHandler};

use app::{BotWrapper, WHandlerWrapper};
use infra::{EventSubWrapper, RepoWrapper, TraqClientWrapper};

pub struct InfraImpl<R, C, S>(pub R, pub C, pub S);

impl<R: Repository, C: TraqClient, S: EventSubscriber> InfraImpl<R, C, S> {
    pub fn new(repo: R, client: C, subscriber: S) -> Self {
        Self(repo, client, subscriber)
    }
}

impl<R: Repository, C: TraqClient, S: EventSubscriber>
    InfraImpl<RepoWrapper<R>, TraqClientWrapper<C>, EventSubWrapper<S>>
where
    domain::Error: From<R::Error>,
    domain::Error: From<C::Error>,
    domain::Error: From<S::Error>,
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
    domain::Error: From<R::Error>,
    C: TraqClient,
    domain::Error: From<C::Error>,
    S: EventSubscriber + Clone,
    domain::Error: From<S::Error>,
{
    type Repo = RepoWrapper<R>;
    type TClient = TraqClientWrapper<C>;
    type ESub = EventSubWrapper<S>;
    type Error = domain::Error;

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
    domain::Error: From<I::Error> + From<B::Error> + From<W::Error>,
{
    pub fn new_wrapped(b: B, w: W) -> AppImpl<BotWrapper<I, B>, WHandlerWrapper<I, W>> {
        let b = BotWrapper::new(b);
        let w = WHandlerWrapper::new(w);
        AppImpl(b, w, PhantomData)
    }
}

impl<I, B, W> App<I> for AppImpl<B, W>
where
    I: Infra<Error = domain::Error>,
    B: Bot<I, Error = domain::Error>,
    W: WebhookHandler<I, Error = domain::Error>,
{
    type Error = domain::Error;
    type Bot = B;
    type WebhookHandler = W;

    fn bot(&self) -> &Self::Bot {
        &self.0
    }

    fn webhook_handler(&self) -> &Self::WebhookHandler {
        &self.1
    }
}
