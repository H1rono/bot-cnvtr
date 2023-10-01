use sqlx::migrate::Migrator;
use sqlx::mysql::MySqlRow;
use sqlx::{MySqlPool, Row};
use uuid::Uuid;

mod config;
pub mod model;

pub use config::Config;

pub use model::group::{Group, GroupRepository};
pub use model::group_member::{GroupMember, GroupMemberRepository};
pub use model::owner::{Owner, OwnerRepository};
pub use model::user::{User, UserRepository};
pub use model::webhook::{Webhook, WebhookRepository};

pub const MIGRATOR: Migrator = sqlx::migrate!("../migrations");

fn parse_col_str2uuid(row: &MySqlRow, col: &str) -> sqlx::Result<Uuid> {
    row.try_get(col).and_then(|u| {
        Uuid::parse_str(u).map_err(|e| sqlx::Error::ColumnDecode {
            index: col.to_string(),
            source: e.into(),
        })
    })
}

pub trait AllRepository: Send + Sync + 'static {
    type GroupMemberRepository: GroupMemberRepository + Send + Sync + 'static;
    type GroupRepository: GroupRepository + Send + Sync + 'static;
    type OwnerRepository: OwnerRepository + Send + Sync + 'static;
    type UserRepository: UserRepository + Send + Sync + 'static;
    type WebhookRepository: WebhookRepository + Send + Sync + 'static;

    fn group_member_repository(&self) -> &Self::GroupMemberRepository;
    fn group_repository(&self) -> &Self::GroupRepository;
    fn owner_repository(&self) -> &Self::OwnerRepository;
    fn user_repository(&self) -> &Self::UserRepository;
    fn webhook_repository(&self) -> &Self::WebhookRepository;
}

pub struct RepositoryImpl(MySqlPool);

impl RepositoryImpl {
    pub fn new(pool: MySqlPool) -> Self {
        Self(pool)
    }

    pub async fn from_config(c: Config) -> sqlx::Result<Self> {
        let url = c.database_url();
        let pool = MySqlPool::connect(&url).await?;
        Ok(Self::new(pool))
    }

    pub async fn migrate(&self) -> sqlx::Result<()> {
        MIGRATOR.run(&self.0).await?;
        Ok(())
    }
}

impl AsRef<MySqlPool> for RepositoryImpl {
    fn as_ref(&self) -> &MySqlPool {
        &self.0
    }
}

impl AllRepository for RepositoryImpl {
    type GroupMemberRepository = Self;
    type GroupRepository = Self;
    type OwnerRepository = Self;
    type UserRepository = Self;
    type WebhookRepository = Self;

    fn group_member_repository(&self) -> &Self::GroupMemberRepository {
        self
    }

    fn group_repository(&self) -> &Self::GroupRepository {
        self
    }

    fn owner_repository(&self) -> &Self::OwnerRepository {
        self
    }

    fn user_repository(&self) -> &Self::UserRepository {
        self
    }

    fn webhook_repository(&self) -> &Self::WebhookRepository {
        self
    }
}
