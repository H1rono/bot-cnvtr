use async_trait::async_trait;
use sqlx::migrate::Migrator;
use sqlx::mysql::MySqlRow;
use sqlx::Result;
use sqlx::{MySqlPool, Row};
use uuid::Uuid;

use config::DbConfig;

mod group;
mod group_member;
mod owner;
mod user;
mod webhook;

pub use group::{Group, GroupDb};
pub use group_member::{GroupMember, GroupMemberDb};
pub use owner::{Owner, OwnerDb};
pub use user::{User, UserDb};
pub use webhook::{Webhook, WebhookDb};

pub const MIGRATOR: Migrator = sqlx::migrate!("../migrations");

#[async_trait]
pub trait Database {
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
    async fn update_webhook(&self, id: &str, w: Webhook) -> Result<()>;
    async fn delete_webhook(&self, id: &Uuid) -> Result<()>;
}

pub struct DatabaseImpl(MySqlPool);

impl DatabaseImpl {
    pub fn new(pool: MySqlPool) -> Self {
        Self(pool)
    }

    pub async fn from_config(c: DbConfig) -> sqlx::Result<Self> {
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
        GroupMemberDb::read_group_members(self).await
    }

    async fn find_group_member(&self, gid: &Uuid, uid: &Uuid) -> Result<Option<GroupMember>> {
        GroupMemberDb::find_group_member(self, gid, uid).await
    }

    async fn filter_group_member_by_gid(&self, gid: &Uuid) -> Result<Vec<GroupMember>> {
        GroupMemberDb::filter_group_member_by_gid(self, gid).await
    }

    async fn filter_group_member_by_uid(&self, uid: &Uuid) -> Result<Vec<GroupMember>> {
        GroupMemberDb::filter_group_member_by_uid(self, uid).await
    }

    async fn create_group_member(&self, gm: GroupMember) -> Result<()> {
        GroupMemberDb::create_group_member(self, gm).await
    }

    async fn create_ignore_group_members(&self, gms: &[GroupMember]) -> Result<()> {
        GroupMemberDb::create_ignore_group_members(self, gms).await
    }

    async fn update_group_member(&self, gid: &Uuid, uid: &Uuid, gm: GroupMember) -> Result<()> {
        GroupMemberDb::update_group_member(self, gid, uid, gm).await
    }

    async fn delete_group_membed(&self, gm: GroupMember) -> Result<()> {
        GroupMemberDb::delete_group_membed(self, gm).await
    }

    // group
    async fn read_groups(&self) -> Result<Vec<Group>> {
        GroupDb::read_groups(self).await
    }

    async fn find_group(&self, id: &Uuid) -> Result<Option<Group>> {
        GroupDb::find_group(self, id).await
    }

    async fn create_group(&self, g: Group) -> Result<()> {
        GroupDb::create_group(self, g).await
    }

    async fn create_ignore_groups(&self, gs: &[Group]) -> Result<()> {
        GroupDb::create_ignore_groups(self, gs).await
    }

    async fn update_group(&self, id: &Uuid, g: Group) -> Result<()> {
        GroupDb::update_group(self, id, g).await
    }

    async fn delete_group(&self, id: &Uuid) -> Result<()> {
        GroupDb::delete_group(self, id).await
    }

    // owner
    async fn read_owners(&self) -> Result<Vec<Owner>> {
        OwnerDb::read_owners(self).await
    }

    async fn find_owner(&self, id: &Uuid) -> Result<Option<Owner>> {
        OwnerDb::find_owner(self, id).await
    }

    async fn create_owner(&self, o: Owner) -> Result<()> {
        OwnerDb::create_owner(self, o).await
    }

    async fn create_ignore_owners(&self, os: &[Owner]) -> Result<()> {
        OwnerDb::create_ignore_owners(self, os).await
    }

    async fn update_owner(&self, id: &Uuid, o: Owner) -> Result<()> {
        OwnerDb::update_owner(self, id, o).await
    }

    async fn delete_owner(&self, id: &Uuid) -> Result<()> {
        OwnerDb::delete_owner(self, id).await
    }

    // user
    async fn read_users(&self) -> Result<Vec<User>> {
        UserDb::read_users(self).await
    }

    async fn find_user(&self, id: &Uuid) -> Result<Option<User>> {
        UserDb::find_user(self, id).await
    }

    async fn create_user(&self, u: User) -> Result<()> {
        UserDb::create_user(self, u).await
    }

    async fn create_ignore_users(&self, us: &[User]) -> Result<()> {
        UserDb::create_ignore_users(self, us).await
    }

    async fn update_user(&self, id: &Uuid, u: User) -> Result<()> {
        UserDb::update_user(self, id, u).await
    }

    async fn delete_user(&self, id: &Uuid) -> Result<()> {
        UserDb::delete_user(self, id).await
    }

    // webhook
    async fn read_webhooks(&self) -> Result<Vec<Webhook>> {
        WebhookDb::read_webhooks(self).await
    }

    async fn find_webhook(&self, id: &Uuid) -> Result<Option<Webhook>> {
        WebhookDb::find_webhook(self, id).await
    }

    async fn filter_webhooks_by_cid(&self, channel_id: Uuid) -> Result<Vec<Webhook>> {
        WebhookDb::filter_webhooks_by_cid(self, channel_id).await
    }

    async fn filter_webhooks_by_oid(&self, owner_id: Uuid) -> Result<Vec<Webhook>> {
        WebhookDb::filter_webhooks_by_oid(self, owner_id).await
    }

    async fn filter_webhooks_by_cids(&self, cids: &[Uuid]) -> Result<Vec<Webhook>> {
        WebhookDb::filter_webhooks_by_cids(self, cids).await
    }

    async fn filter_webhooks_by_oids(&self, oids: &[Uuid]) -> Result<Vec<Webhook>> {
        WebhookDb::filter_webhooks_by_oids(self, oids).await
    }

    async fn create_webhook(&self, w: Webhook) -> Result<()> {
        WebhookDb::create_webhook(self, w).await
    }

    async fn create_ignore_webhooks(&self, ws: &[Webhook]) -> Result<()> {
        WebhookDb::create_ignore_webhooks(self, ws).await
    }

    async fn update_webhook(&self, id: &str, w: Webhook) -> Result<()> {
        WebhookDb::update_webhook(self, id, w).await
    }

    async fn delete_webhook(&self, id: &Uuid) -> Result<()> {
        WebhookDb::delete_webhook(self, id).await
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
