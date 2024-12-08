pub mod app;
pub mod infra;

use domain::{EventSubscriber, Infra, Repository, TraqClient};

// use app::{BotWrapper, WHandlerWrapper};
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
