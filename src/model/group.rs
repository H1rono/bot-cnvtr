use indoc::indoc;
use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlRow, FromRow, Result, Row};
use uuid::Uuid;

use super::{parse_col_str2uuid, Database};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Group {
    pub id: Uuid,
    pub name: String,
}

impl<'r> FromRow<'r, MySqlRow> for Group {
    fn from_row(row: &'r MySqlRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(Self {
            id: parse_col_str2uuid(row, "id")?,
            name: row.try_get("name")?,
        })
    }
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

    pub async fn find_group(&self, id: &Uuid) -> Result<Option<Group>> {
        sqlx::query(indoc! {r#"
            SELECT *
            FROM `groups`
            WHERE `id` = ?
            LIMIT 1
        "#})
        .bind(id.to_string())
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
        .bind(g.id.to_string())
        .bind(g.name)
        .execute(&self.0)
        .await?;
        Ok(())
    }

    pub async fn update_group(&self, id: &Uuid, g: Group) -> Result<()> {
        sqlx::query(indoc! {r#"
            UPDATE `groups`
            SET `id` = ?, `name` = ?
            WHERE `id` = ?
        "#})
        .bind(g.id.to_string())
        .bind(g.name)
        .bind(id.to_string())
        .execute(&self.0)
        .await?;
        Ok(())
    }

    pub async fn delete_group(&self, id: &Uuid) -> Result<()> {
        sqlx::query(indoc! {r#"
            DELETE FROM `groups`
            WHERE `id` = ?
        "#})
        .bind(id.to_string())
        .execute(&self.0)
        .await?;
        Ok(())
    }
}
