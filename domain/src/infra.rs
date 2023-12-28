use crate::repository::Repository;
use crate::traq_client::TraqClient;

pub trait Infra: Send + Sync + 'static {
    type Repo: Repository;
    type TClient: TraqClient;

    fn repo(&self) -> &Self::Repo;
    fn traq_client(&self) -> &Self::TClient;
}
