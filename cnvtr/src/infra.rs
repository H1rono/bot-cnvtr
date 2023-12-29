use domain::{Infra, Repository, TraqClient};

pub struct InfraImpl<R, C>(pub R, pub C);

impl<R, C, E1, E2> Infra for InfraImpl<R, C>
where
    R: Repository<Error = E1>,
    C: TraqClient<Error = E2>,
    usecases::Error: From<E1> + From<E2>,
{
    type Repo = R;
    type TClient = C;
    type Error = usecases::Error;

    fn repo(&self) -> &Self::Repo {
        &self.0
    }

    fn traq_client(&self) -> &Self::TClient {
        &self.1
    }
}
