use std::iter;

use indoc::{formatdoc, indoc};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlRow, FromRow, Result, Row};
use uuid::Uuid;

use super::parse_col_str2uuid;
use crate::RepositoryImpl;

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

#[allow(dead_code)]
impl RepositoryImpl {
    pub(crate) async fn read_groups(&self) -> Result<Vec<Group>> {
        sqlx::query_as(indoc! {r"
            SELECT *
            FROM `groups`
        "})
        .fetch_all(&self.0)
        .await
    }

    pub(crate) async fn find_group(&self, id: &Uuid) -> Result<Option<Group>> {
        sqlx::query_as(indoc! {r"
            SELECT *
            FROM `groups`
            WHERE `id` = ?
            LIMIT 1
        "})
        .bind(id.to_string())
        .fetch_optional(&self.0)
        .await
    }

    pub(crate) async fn create_group(&self, g: Group) -> Result<()> {
        sqlx::query(indoc! {r"
            INSERT INTO `groups` (`id`, `name`)
            VALUES (?, ?)
        "})
        .bind(g.id.to_string())
        .bind(g.name)
        .execute(&self.0)
        .await?;
        Ok(())
    }

    pub(crate) async fn create_ignore_groups(&self, gs: &[Group]) -> Result<()> {
        if gs.is_empty() {
            return Ok(());
        }
        let query = formatdoc! {
            r"
                INSERT IGNORE
                INTO `groups` (`id`, `name`)
                VALUES {}
            ",
            iter::repeat("(?, ?)").take(gs.len()).join(", ")
        };
        let query = gs.iter().fold(sqlx::query(&query), |q, g| {
            q.bind(g.id.to_string()).bind(&g.name)
        });
        query.execute(&self.0).await?;
        Ok(())
    }

    pub(crate) async fn update_group(&self, id: &Uuid, g: Group) -> Result<()> {
        sqlx::query(indoc! {r"
            UPDATE `groups`
            SET `id` = ?, `name` = ?
            WHERE `id` = ?
        "})
        .bind(g.id.to_string())
        .bind(g.name)
        .bind(id.to_string())
        .execute(&self.0)
        .await?;
        Ok(())
    }

    pub(crate) async fn delete_group(&self, id: &Uuid) -> Result<()> {
        sqlx::query(indoc! {r"
            DELETE FROM `groups`
            WHERE `id` = ?
        "})
        .bind(id.to_string())
        .execute(&self.0)
        .await?;
        Ok(())
    }
}
