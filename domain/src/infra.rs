use crate::repository::Repository;
use crate::traq_client::TraqClient;

pub trait Infra: Send + Sync + 'static
where
    Self::Error:
        From<<Self::Repo as Repository>::Error> + From<<Self::TClient as TraqClient>::Error>,
{
    type Repo: Repository;
    type TClient: TraqClient;
    type Error: Send + Sync + 'static;

    fn repo(&self) -> &Self::Repo;
    fn traq_client(&self) -> &Self::TClient;
}
