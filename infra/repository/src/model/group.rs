use std::iter;

use domain::{GroupId, UserId};
use indoc::formatdoc;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlRow, FromRow};
use uuid::Uuid;

use crate::error::{Error, Result};
use crate::RepositoryImpl;

const TABLE_GROUPS: &str = "groups_v2";

#[must_use]
#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
struct GroupRow {
    pub id: Uuid,
    pub name: String,
}

#[must_use]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Group {
    pub id: GroupId,
    pub name: String,
}

impl From<GroupRow> for Group {
    fn from(value: GroupRow) -> Self {
        let GroupRow { id, name } = value;
        #[allow(clippy::useless_conversion)]
        Self {
            id: id.into(),
            name: name.into(),
        }
    }
}

impl From<Group> for GroupRow {
    fn from(value: Group) -> Self {
        let Group { id, name } = value;
        #[allow(clippy::useless_conversion)]
        Self {
            id: id.into(),
            name: name.into(),
        }
    }
}

impl<'r> FromRow<'r, MySqlRow> for Group {
    fn from_row(row: &'r MySqlRow) -> std::result::Result<Self, sqlx::Error> {
        GroupRow::from_row(row).map(Self::from)
    }
}

#[allow(dead_code)]
impl RepositoryImpl {
    pub(crate) async fn read_groups(&self) -> Result<Vec<Group>> {
        let query = formatdoc! {r"
            SELECT *
            FROM `{TABLE_GROUPS}`
        "};
        sqlx::query_as(&query)
            .fetch_all(&self.0)
            .await
            .map_err(Error::from)
    }

    pub(crate) async fn find_group(&self, id: &GroupId) -> Result<Option<Group>> {
        let query = formatdoc! {r"
            SELECT *
            FROM `{TABLE_GROUPS}`
            WHERE `id` = ?
            LIMIT 1
        "};
        sqlx::query_as(&query)
            .bind(id.to_string())
            .fetch_optional(&self.0)
            .await
            .map_err(Error::from)
    }

    pub(crate) async fn create_group(&self, g: Group) -> Result<()> {
        let query = formatdoc! {r"
            INSERT INTO `{TABLE_GROUPS}` (`id`, `name`)
            VALUES (?, ?)
        "};
        sqlx::query(&query)
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
        let values_arg = iter::repeat("(?, ?)").take(gs.len()).join(", ");
        let query = formatdoc! {r"
            INSERT IGNORE
            INTO `{TABLE_GROUPS}` (`id`, `name`)
            VALUES {values_arg}
        "};
        let query = gs.iter().fold(sqlx::query(&query), |q, g| {
            q.bind(g.id.to_string()).bind(&g.name)
        });
        query.execute(&self.0).await?;
        Ok(())
    }

    pub(crate) async fn update_group(&self, id: &UserId, g: Group) -> Result<()> {
        let query = formatdoc! {r"
            UPDATE `{TABLE_GROUPS}`
            SET `id` = ?, `name` = ?
            WHERE `id` = ?
        "};
        sqlx::query(&query)
            .bind(g.id.to_string())
            .bind(g.name)
            .bind(id.to_string())
            .execute(&self.0)
            .await?;
        Ok(())
    }

    pub(crate) async fn delete_group(&self, id: &GroupId) -> Result<()> {
        let query = formatdoc! {r"
            DELETE FROM `{TABLE_GROUPS}`
            WHERE `id` = ?
        "};
        sqlx::query(&query)
            .bind(id.to_string())
            .execute(&self.0)
            .await?;
        Ok(())
    }
}
