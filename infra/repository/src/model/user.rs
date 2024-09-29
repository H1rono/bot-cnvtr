use std::iter;

use indoc::formatdoc;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlRow, FromRow};
use uuid::Uuid;

use domain::UserId;

use crate::error::{Error, Result};
use crate::RepositoryImpl;

const TABLE_USERS: &str = "users_v2";

#[must_use]
#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
struct UserRow {
    pub id: Uuid,
    pub name: String,
}

#[must_use]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct User {
    pub id: UserId,
    pub name: String,
}

impl From<UserRow> for User {
    fn from(value: UserRow) -> Self {
        let UserRow { id, name } = value;
        #[allow(clippy::useless_conversion)]
        Self {
            id: id.into(),
            name: name.into(),
        }
    }
}

impl<'r> FromRow<'r, MySqlRow> for User {
    fn from_row(row: &'r MySqlRow) -> std::result::Result<Self, sqlx::Error> {
        UserRow::from_row(row).map(Self::from)
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
            .bind(id.0)
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
            .bind(u.id.0)
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
        let query = us
            .iter()
            .fold(sqlx::query(&query), |q, u| q.bind(u.id.0).bind(&u.name));
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
            .bind(u.id.0)
            .bind(u.name)
            .bind(id.0)
            .execute(&self.0)
            .await?;
        Ok(())
    }

    pub(crate) async fn delete_user(&self, id: &UserId) -> Result<()> {
        let query = formatdoc! {r"
            DELETE FROM `{TABLE_USERS}`
            WHERE `id` = ?
        "};
        sqlx::query(&query).bind(id.0).execute(&self.0).await?;
        Ok(())
    }
}
