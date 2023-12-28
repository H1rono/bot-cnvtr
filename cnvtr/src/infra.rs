use domain::{Infra, Repository, TraqClient};

pub struct InfraImpl<R, C>(pub R, pub C);

impl<R, C> Infra for InfraImpl<R, C>
where
    R: Repository,
    C: TraqClient,
{
    type Repo = R;
    type TClient = C;

    fn repo(&self) -> &Self::Repo {
        &self.0
    }

    fn traq_client(&self) -> &Self::TClient {
        &self.1
    }
}
