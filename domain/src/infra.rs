use crate::repository::Repository;
use crate::traq_client::TraqClient;

pub trait Infra: Send + Sync + 'static {
    type Error: Into<crate::error::Error> + Send + Sync + 'static;
    type Repo: Repository<Error = Self::Error>;
    type TClient: TraqClient<Error = Self::Error>;

    fn repo(&self) -> &Self::Repo;
    fn traq_client(&self) -> &Self::TClient;
}
