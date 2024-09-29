use std::iter;

use domain::UserId;
use indoc::formatdoc;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlRow, FromRow, Row};

use super::parse_col_str2uuid;
use crate::error::{Error, Result};
use crate::RepositoryImpl;

const TABLE_USERS: &str = "users";

#[must_use]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct User {
    pub id: UserId,
    pub name: String,
}

impl<'r> FromRow<'r, MySqlRow> for User {
    fn from_row(row: &'r MySqlRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(Self {
            id: parse_col_str2uuid(row, "id")?,
            name: row.try_get("name")?,
        })
    }
}

#[allow(dead_code)]
impl RepositoryImpl {
    pub(crate) async fn read_users(&self) -> Result<Vec<User>> {
        let query = formatdoc! {r"
            SELECT *
            FROM `{TABLE_USERS}`
        "};
        sqlx::query_as(&query)
            .fetch_all(&self.0)
            .await
            .map_err(Error::from)
    }

    pub(crate) async fn find_user(&self, id: &UserId) -> Result<Option<User>> {
        let query = formatdoc! {r"
            SELECT *
            FROM `{TABLE_USERS}`
            WHERE `id` = ?
            LIMIT 1
        "};
        sqlx::query_as(&query)
            .bind(id.to_string())
            .fetch_optional(&self.0)
            .await
            .map_err(Error::from)
    }

    pub(crate) async fn create_user(&self, u: User) -> Result<()> {
        let query = formatdoc! {r"
            INSERT INTO `{TABLE_USERS}` (`id`, `name`)
            VALUES (?, ?)
        "};
        sqlx::query(&query)
            .bind(u.id.to_string())
            .bind(u.name)
            .execute(&self.0)
            .await?;
        Ok(())
    }

    pub(crate) async fn create_ignore_users(&self, us: &[User]) -> Result<()> {
        if us.is_empty() {
            return Ok(());
        }
        let values_arg = iter::repeat("(?, ?)").take(us.len()).join(", ");
        let query = formatdoc! {r"
            INSERT IGNORE
            INTO `{TABLE_USERS}` (`id`, `name`)
            VALUES {values_arg}
        "};
        let query = us.iter().fold(sqlx::query(&query), |q, u| {
            q.bind(u.id.to_string()).bind(&u.name)
        });
        query.execute(&self.0).await?;
        Ok(())
    }

    pub(crate) async fn update_user(&self, id: &UserId, u: User) -> Result<()> {
        let query = formatdoc! {r"
            UPDATE `{TABLE_USERS}`
            SET `id` = ?, `name` = ?
            WHERE `id` = ?
        "};
        sqlx::query(&query)
            .bind(u.id.to_string())
            .bind(u.name)
            .bind(id.to_string())
            .execute(&self.0)
            .await?;
        Ok(())
    }

    pub(crate) async fn delete_user(&self, id: &UserId) -> Result<()> {
        let query = formatdoc! {r"
            DELETE FROM `{TABLE_USERS}`
            WHERE `id` = ?
        "};
        sqlx::query(&query)
            .bind(id.to_string())
            .execute(&self.0)
            .await?;
        Ok(())
    }
}
