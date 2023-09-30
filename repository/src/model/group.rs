use std::iter;

use async_trait::async_trait;
use indoc::{formatdoc, indoc};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlRow, FromRow, Result, Row};
use uuid::Uuid;

use crate::{parse_col_str2uuid, DatabaseImpl};

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

#[async_trait]
pub trait GroupRepository {
    async fn read(&self) -> Result<Vec<Group>>;
    async fn find(&self, id: &Uuid) -> Result<Option<Group>>;
    async fn create(&self, g: Group) -> Result<()>;
    async fn create_ignore(&self, gs: &[Group]) -> Result<()>;
    async fn update(&self, id: &Uuid, g: Group) -> Result<()>;
    async fn delete(&self, id: &Uuid) -> Result<()>;
}

#[async_trait]
impl GroupRepository for DatabaseImpl {
    async fn read(&self) -> Result<Vec<Group>> {
        sqlx::query_as(indoc! {r#"
            SELECT *
            FROM `groups`
        "#})
        .fetch_all(&self.0)
        .await
    }

    async fn find(&self, id: &Uuid) -> Result<Option<Group>> {
        sqlx::query_as(indoc! {r#"
            SELECT *
            FROM `groups`
            WHERE `id` = ?
            LIMIT 1
        "#})
        .bind(id.to_string())
        .fetch_optional(&self.0)
        .await
    }

    async fn create(&self, g: Group) -> Result<()> {
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

    async fn create_ignore(&self, gs: &[Group]) -> Result<()> {
        let gs_len = gs.len();
        if gs_len == 0 {
            return Ok(());
        }
        let query = formatdoc! {
            r#"
                INSERT IGNORE
                INTO `groups` (`id`, `name`)
                VALUES {}
            "#,
            iter::repeat("(?, ?)").take(gs_len).join(", ")
        };
        let query = gs.iter().fold(sqlx::query(&query), |q, g| {
            q.bind(g.id.to_string()).bind(&g.name)
        });
        query.execute(&self.0).await?;
        Ok(())
    }

    async fn update(&self, id: &Uuid, g: Group) -> Result<()> {
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

    async fn delete(&self, id: &Uuid) -> Result<()> {
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
