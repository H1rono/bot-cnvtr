use std::iter;

use domain::OwnerId;
use indoc::formatdoc;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlRow, FromRow, Row};

use super::parse_col_str2uuid;
use crate::error::{Error, Result};
use crate::RepositoryImpl;

const TABLE_OWNERS: &str = "owners";

#[must_use]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Owner {
    pub id: OwnerId,
    pub name: String,
    pub group: bool,
}

impl<'r> FromRow<'r, MySqlRow> for Owner {
    fn from_row(row: &'r MySqlRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(Self {
            id: parse_col_str2uuid(row, "id")?,
            name: row.try_get("name")?,
            group: row.try_get("group")?,
        })
    }
}

#[allow(dead_code)]
impl RepositoryImpl {
    pub(crate) async fn read_owners(&self) -> Result<Vec<Owner>> {
        let query = formatdoc! {r"
            SELECT *
            FROM `{TABLE_OWNERS}`
        "};
        sqlx::query_as(&query)
            .fetch_all(&self.0)
            .await
            .map_err(Error::from)
    }

    pub(crate) async fn find_owner(&self, id: &OwnerId) -> Result<Option<Owner>> {
        let query = formatdoc! {r"
            SELECT *
            FROM `{TABLE_OWNERS}`
            WHERE `id` = ?
            LIMIT 1
        "};
        sqlx::query_as(&query)
            .bind(id.to_string())
            .fetch_optional(&self.0)
            .await
            .map_err(Error::from)
    }

    pub(crate) async fn create_owner(&self, o: Owner) -> Result<()> {
        let query = formatdoc! {r"
            INSERT INTO `{TABLE_OWNERS}` (`id`, `name`, `group`)
            VALUES (?, ?, ?)
        "};
        sqlx::query(&query)
            .bind(o.id.to_string())
            .bind(o.name)
            .bind(o.group)
            .execute(&self.0)
            .await?;
        Ok(())
    }

    pub(crate) async fn create_ignore_owners(&self, os: &[Owner]) -> Result<()> {
        if os.is_empty() {
            return Ok(());
        }
        let values_arg = iter::repeat("(?, ?, ?)").take(os.len()).join(", ");
        let query = formatdoc! {r"
            INSERT IGNORE
            INTO `{TABLE_OWNERS}` (`id`, `name`, `group`)
            VALUES {values_arg}
        "};
        let query = os.iter().fold(sqlx::query(&query), |q, o| {
            q.bind(o.id.to_string()).bind(&o.name).bind(o.group)
        });
        query.execute(&self.0).await?;
        Ok(())
    }

    pub(crate) async fn update_owner(&self, id: &OwnerId, o: Owner) -> Result<()> {
        let query = formatdoc! {r"
            UPDATE `{TABLE_OWNERS}`
            SET `id` = ?, `name` = ?, `group` = ?
            WHERE `id` = ?
        "};
        sqlx::query(&query)
            .bind(o.id.to_string())
            .bind(o.name)
            .bind(o.group)
            .bind(id.to_string())
            .execute(&self.0)
            .await?;
        Ok(())
    }

    pub(crate) async fn delete_owner(&self, id: &OwnerId) -> Result<()> {
        let query = formatdoc! {r"
            DELETE FROM `{TABLE_OWNERS}`
            WHERE `id` = ?
        "};
        sqlx::query(&query)
            .bind(id.to_string())
            .execute(&self.0)
            .await?;
        Ok(())
    }
}
