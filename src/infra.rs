pub mod event_subscriber;
pub mod repository;
pub mod traq_client;

use domain::{EventSubscriber, Infra, Repository, TraqClient};

use event_subscriber::EventSubWrapper;
use repository::RepoWrapper;
use traq_client::TraqClientWrapper;

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
