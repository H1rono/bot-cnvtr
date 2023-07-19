use sqlx::migrate::Migrator;
use sqlx::mysql::MySqlRow;
use sqlx::{MySqlPool, Row};
use uuid::Uuid;

use super::config::DbConfig;

mod group;
mod group_member;
mod owner;
mod user;
mod webhook;

pub use group::Group;
pub use group_member::GroupMember;
pub use owner::Owner;
pub use user::User;
pub use webhook::Webhook;

pub const MIGRATOR: Migrator = sqlx::migrate!("./migrations");

pub struct Database(MySqlPool);

impl Database {
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

fn parse_col_str2uuid(row: &MySqlRow, col: &str) -> sqlx::Result<Uuid> {
    row.try_get(col).and_then(|u| {
        Uuid::parse_str(u).map_err(|e| sqlx::Error::ColumnDecode {
            index: col.to_string(),
            source: e.into(),
        })
    })
}
