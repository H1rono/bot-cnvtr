use domain::{Infra, Repository, TraqClient};

use crate::repo::RepoWrapper;
use crate::traq_client::TraqClientWrapper;

pub struct InfraImpl<R, C>(pub R, pub C);

impl<R: Repository, C: TraqClient> InfraImpl<R, C> {
    pub fn new(repo: R, client: C) -> Self {
        Self(repo, client)
    }
}

impl<R: Repository, C: TraqClient> InfraImpl<RepoWrapper<R>, TraqClientWrapper<C>>
where
    R::Error: Into<usecases::Error>,
    C::Error: Into<usecases::Error>,
{
    pub fn new_wrapped(repo: R, client: C) -> Self {
        let repo = RepoWrapper(repo);
        let client = TraqClientWrapper(client);
        Self(repo, client)
    }
}

impl<R, C> Infra for InfraImpl<RepoWrapper<R>, TraqClientWrapper<C>>
where
    R: Repository,
    usecases::Error: From<R::Error>,
    C: TraqClient,
    usecases::Error: From<C::Error>,
{
    type Repo = RepoWrapper<R>;
    type TClient = TraqClientWrapper<C>;
    type Error = usecases::Error;

    fn repo(&self) -> &Self::Repo {
        &self.0
    }

    fn traq_client(&self) -> &Self::TClient {
        &self.1
    }
}
