use sqlx::migrate::Migrator;
use sqlx::MySqlPool;

use super::DbConfig;

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
