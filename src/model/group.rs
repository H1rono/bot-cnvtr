use indoc::indoc;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Result};

use super::Database;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, FromRow)]
pub struct Group {
    pub id: String,
    pub name: String,
}

impl Database {
    pub async fn read_groups(&self) -> Result<Vec<Group>> {
        sqlx::query(indoc! {r#"
            SELECT *
            FROM `groups`
        "#})
        .fetch_all(&self.0)
        .await?
        .iter()
        .map(Group::from_row)
        .collect::<Result<_>>()
    }

    pub async fn find_group(&self, id: &str) -> Result<Option<Group>> {
        sqlx::query(indoc! {r#"
            SELECT *
            FROM `groups`
            WHERE `id` = ?
            LIMIT 1
        "#})
        .bind(id)
        .fetch_optional(&self.0)
        .await?
        .map(|g| Group::from_row(&g))
        .transpose()
    }

    pub async fn create_group(&self, g: Group) -> Result<()> {
        sqlx::query(indoc! {r#"
            INSERT INTO `groups` (`id`, `name`)
            VALUES (?, ?)
        "#})
        .bind(g.id)
        .bind(g.name)
        .execute(&self.0)
        .await?;
        Ok(())
    }

    pub async fn update_group(&self, id: &str, g: Group) -> Result<()> {
        sqlx::query(indoc! {r#"
            UPDATE `groups`
            SET `id` = ?, `name` = ?
            WHERE `id` = ?
        "#})
        .bind(g.id)
        .bind(g.name)
        .bind(id)
        .execute(&self.0)
        .await?;
        Ok(())
    }

    pub async fn delete_group(&self, id: &str) -> Result<()> {
        sqlx::query(indoc! {r#"
            DELETE FROM `groups`
            WHERE `id` = ?
        "#})
        .bind(id)
        .execute(&self.0)
        .await?;
        Ok(())
    }
}
