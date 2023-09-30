use async_trait::async_trait;
use sqlx::migrate::Migrator;
use sqlx::mysql::MySqlRow;
use sqlx::Result;
use sqlx::{MySqlPool, Row};
use uuid::Uuid;

use config::DbConfig as Config;

pub mod model;

pub use model::group::{Group, GroupRepository};
pub use model::group_member::{GroupMember, GroupMemberRepository};
pub use model::owner::{Owner, OwnerRepository};
pub use model::user::{User, UserRepository};
pub use model::webhook::{Webhook, WebhookRepository};

pub const MIGRATOR: Migrator = sqlx::migrate!("../migrations");

#[async_trait]
pub trait Database: Sync + Send + 'static {
    // group_member
    async fn read_group_members(&self) -> Result<Vec<GroupMember>>;
    async fn find_group_member(&self, gid: &Uuid, uid: &Uuid) -> Result<Option<GroupMember>>;
    async fn filter_group_member_by_gid(&self, gid: &Uuid) -> Result<Vec<GroupMember>>;
    async fn filter_group_member_by_uid(&self, uid: &Uuid) -> Result<Vec<GroupMember>>;
    async fn create_group_member(&self, gm: GroupMember) -> Result<()>;
    async fn create_ignore_group_members(&self, gms: &[GroupMember]) -> Result<()>;
    async fn update_group_member(&self, gid: &Uuid, uid: &Uuid, gm: GroupMember) -> Result<()>;
    async fn delete_group_membed(&self, gm: GroupMember) -> Result<()>;
    // group
    async fn read_groups(&self) -> Result<Vec<Group>>;
    async fn find_group(&self, id: &Uuid) -> Result<Option<Group>>;
    async fn create_group(&self, g: Group) -> Result<()>;
    async fn create_ignore_groups(&self, gs: &[Group]) -> Result<()>;
    async fn update_group(&self, id: &Uuid, g: Group) -> Result<()>;
    async fn delete_group(&self, id: &Uuid) -> Result<()>;
    // owner
    async fn read_owners(&self) -> Result<Vec<Owner>>;
    async fn find_owner(&self, id: &Uuid) -> Result<Option<Owner>>;
    async fn create_owner(&self, o: Owner) -> Result<()>;
    async fn create_ignore_owners(&self, os: &[Owner]) -> Result<()>;
    async fn update_owner(&self, id: &Uuid, o: Owner) -> Result<()>;
    async fn delete_owner(&self, id: &Uuid) -> Result<()>;
    // user
    async fn read_users(&self) -> Result<Vec<User>>;
    async fn find_user(&self, id: &Uuid) -> Result<Option<User>>;
    async fn create_user(&self, u: User) -> Result<()>;
    async fn create_ignore_users(&self, us: &[User]) -> Result<()>;
    async fn update_user(&self, id: &Uuid, u: User) -> Result<()>;
    async fn delete_user(&self, id: &Uuid) -> Result<()>;
    // webhook
    async fn read_webhooks(&self) -> Result<Vec<Webhook>>;
    async fn find_webhook(&self, id: &Uuid) -> Result<Option<Webhook>>;
    async fn filter_webhooks_by_cid(&self, channel_id: Uuid) -> Result<Vec<Webhook>>;
    async fn filter_webhooks_by_oid(&self, owner_id: Uuid) -> Result<Vec<Webhook>>;
    async fn filter_webhooks_by_cids(&self, cids: &[Uuid]) -> Result<Vec<Webhook>>;
    async fn filter_webhooks_by_oids(&self, oids: &[Uuid]) -> Result<Vec<Webhook>>;
    async fn create_webhook(&self, w: Webhook) -> Result<()>;
    async fn create_ignore_webhooks(&self, ws: &[Webhook]) -> Result<()>;
    async fn update_webhook(&self, id: &Uuid, w: Webhook) -> Result<()>;
    async fn delete_webhook(&self, id: &Uuid) -> Result<()>;
}

pub struct DatabaseImpl(MySqlPool);

impl DatabaseImpl {
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

impl AsRef<MySqlPool> for DatabaseImpl {
    fn as_ref(&self) -> &MySqlPool {
        &self.0
    }
}

#[async_trait]
impl Database for DatabaseImpl {
    // group_member
    async fn read_group_members(&self) -> Result<Vec<GroupMember>> {
        GroupMemberRepository::read(self).await
    }

    async fn find_group_member(&self, gid: &Uuid, uid: &Uuid) -> Result<Option<GroupMember>> {
        GroupMemberRepository::find(self, gid, uid).await
    }

    async fn filter_group_member_by_gid(&self, gid: &Uuid) -> Result<Vec<GroupMember>> {
        GroupMemberRepository::filter_by_gid(self, gid).await
    }

    async fn filter_group_member_by_uid(&self, uid: &Uuid) -> Result<Vec<GroupMember>> {
        GroupMemberRepository::filter_by_uid(self, uid).await
    }

    async fn create_group_member(&self, gm: GroupMember) -> Result<()> {
        GroupMemberRepository::create(self, gm).await
    }

    async fn create_ignore_group_members(&self, gms: &[GroupMember]) -> Result<()> {
        GroupMemberRepository::create_ignore(self, gms).await
    }

    async fn update_group_member(&self, gid: &Uuid, uid: &Uuid, gm: GroupMember) -> Result<()> {
        GroupMemberRepository::update(self, gid, uid, gm).await
    }

    async fn delete_group_membed(&self, gm: GroupMember) -> Result<()> {
        GroupMemberRepository::delete(self, gm).await
    }

    // group
    async fn read_groups(&self) -> Result<Vec<Group>> {
        GroupRepository::read(self).await
    }

    async fn find_group(&self, id: &Uuid) -> Result<Option<Group>> {
        GroupRepository::find(self, id).await
    }

    async fn create_group(&self, g: Group) -> Result<()> {
        GroupRepository::create(self, g).await
    }

    async fn create_ignore_groups(&self, gs: &[Group]) -> Result<()> {
        GroupRepository::create_ignore(self, gs).await
    }

    async fn update_group(&self, id: &Uuid, g: Group) -> Result<()> {
        GroupRepository::update(self, id, g).await
    }

    async fn delete_group(&self, id: &Uuid) -> Result<()> {
        GroupRepository::delete(self, id).await
    }

    // owner
    async fn read_owners(&self) -> Result<Vec<Owner>> {
        OwnerRepository::read(self).await
    }

    async fn find_owner(&self, id: &Uuid) -> Result<Option<Owner>> {
        OwnerRepository::find(self, id).await
    }

    async fn create_owner(&self, o: Owner) -> Result<()> {
        OwnerRepository::create(self, o).await
    }

    async fn create_ignore_owners(&self, os: &[Owner]) -> Result<()> {
        OwnerRepository::create_ignore(self, os).await
    }

    async fn update_owner(&self, id: &Uuid, o: Owner) -> Result<()> {
        OwnerRepository::update(self, id, o).await
    }

    async fn delete_owner(&self, id: &Uuid) -> Result<()> {
        OwnerRepository::delete(self, id).await
    }

    // user
    async fn read_users(&self) -> Result<Vec<User>> {
        UserRepository::read(self).await
    }

    async fn find_user(&self, id: &Uuid) -> Result<Option<User>> {
        UserRepository::find(self, id).await
    }

    async fn create_user(&self, u: User) -> Result<()> {
        UserRepository::create(self, u).await
    }

    async fn create_ignore_users(&self, us: &[User]) -> Result<()> {
        UserRepository::create_ignore(self, us).await
    }

    async fn update_user(&self, id: &Uuid, u: User) -> Result<()> {
        UserRepository::update(self, id, u).await
    }

    async fn delete_user(&self, id: &Uuid) -> Result<()> {
        UserRepository::delete(self, id).await
    }

    // webhook
    async fn read_webhooks(&self) -> Result<Vec<Webhook>> {
        WebhookRepository::read(self).await
    }

    async fn find_webhook(&self, id: &Uuid) -> Result<Option<Webhook>> {
        WebhookRepository::find(self, id).await
    }

    async fn filter_webhooks_by_cid(&self, channel_id: Uuid) -> Result<Vec<Webhook>> {
        WebhookRepository::filter_by_cid(self, channel_id).await
    }

    async fn filter_webhooks_by_oid(&self, owner_id: Uuid) -> Result<Vec<Webhook>> {
        WebhookRepository::filter_by_oid(self, owner_id).await
    }

    async fn filter_webhooks_by_cids(&self, cids: &[Uuid]) -> Result<Vec<Webhook>> {
        WebhookRepository::filter_by_cids(self, cids).await
    }

    async fn filter_webhooks_by_oids(&self, oids: &[Uuid]) -> Result<Vec<Webhook>> {
        WebhookRepository::filter_by_oids(self, oids).await
    }

    async fn create_webhook(&self, w: Webhook) -> Result<()> {
        WebhookRepository::create(self, w).await
    }

    async fn create_ignore_webhooks(&self, ws: &[Webhook]) -> Result<()> {
        WebhookRepository::create_ignore(self, ws).await
    }

    async fn update_webhook(&self, id: &Uuid, w: Webhook) -> Result<()> {
        WebhookRepository::update(self, id, w).await
    }

    async fn delete_webhook(&self, id: &Uuid) -> Result<()> {
        WebhookRepository::delete(self, id).await
    }
}

fn parse_col_str2uuid(row: &MySqlRow, col: &str) -> sqlx::Result<Uuid> {
    row.try_get(col).and_then(|u| {
        Uuid::parse_str(u).map_err(|e| sqlx::Error::ColumnDecode {
            index: col.to_string(),
            source: e.into(),
        })
    })
}
