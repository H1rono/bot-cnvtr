use indoc::indoc;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Result};

use super::Database;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, FromRow)]
pub struct Owner {
    pub id: String,
    pub name: String,
    pub group: bool,
}

impl Database {
    pub async fn read_owners(&self) -> Result<Vec<Owner>> {
        sqlx::query(indoc! {r#"
            SELECT *
            FROM `owners`
        "#})
        .fetch_all(&self.0)
        .await?
        .iter()
        .map(Owner::from_row)
        .collect::<Result<_>>()
    }

    pub async fn create_owner(&self, o: Owner) -> Result<()> {
        sqlx::query(indoc! {r#"
            INSERT INTO `owners` (`id`, `name`, `group`)
            VALUES (?, ?, ?)
        "#})
        .bind(o.id)
        .bind(o.name)
        .bind(o.group)
        .execute(&self.0)
        .await?;
        Ok(())
    }

    pub async fn update_owner(&self, id: &str, o: Owner) -> Result<()> {
        sqlx::query(indoc! {r#"
            UPDATE `owners`
            SET `id` = ?, `name` = ?, `group` = ?
            WHERE `id` = ?
        "#})
        .bind(o.id)
        .bind(o.name)
        .bind(o.group)
        .bind(id)
        .execute(&self.0)
        .await?;
        Ok(())
    }

    pub async fn delete_owner(&self, id: &str) -> Result<()> {
        sqlx::query(indoc! {r#"
            DELETE FROM `owners`
            WHERE `id` = ?
        "#})
        .bind(id)
        .execute(&self.0)
        .await?;
        Ok(())
    }
}
