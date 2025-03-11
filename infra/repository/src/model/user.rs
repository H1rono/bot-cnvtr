use std::iter;

use anyhow::Context;
use indoc::formatdoc;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, mysql::MySqlRow};
use uuid::Uuid;

use domain::{Failure, UserId};

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
    fn from_row(row: &'r MySqlRow) -> sqlx::Result<Self> {
        UserRow::from_row(row).map(Self::from)
    }
}

#[allow(dead_code)]
impl RepositoryImpl {
    pub(crate) async fn read_users(&self) -> Result<Vec<User>, Failure> {
        let query = formatdoc! {r"
            SELECT *
            FROM `{TABLE_USERS}`
        "};
        let res = sqlx::query_as(&query)
            .fetch_all(&self.0)
            .await
            .context("Failed to read users from DB")?;
        Ok(res)
    }

    pub(crate) async fn find_user(&self, id: &UserId) -> Result<User, Failure> {
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
            .context("Failed to read an user from DB")?
            .ok_or_else(|| Failure::reject_not_found("No user found"))
    }

    pub(crate) async fn create_user(&self, u: User) -> Result<(), Failure> {
        let query = formatdoc! {r"
            INSERT INTO `{TABLE_USERS}` (`id`, `name`)
            VALUES (?, ?)
        "};
        sqlx::query(&query)
            .bind(u.id.0)
            .bind(u.name)
            .execute(&self.0)
            .await
            .context("Failed to create an user to DB")?;
        Ok(())
    }

    pub(crate) async fn create_ignore_users(&self, us: &[User]) -> Result<(), Failure> {
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
        query
            .execute(&self.0)
            .await
            .context("Failed to create users to DB")?;
        Ok(())
    }

    pub(crate) async fn update_user(&self, id: &UserId, u: User) -> Result<(), Failure> {
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
            .await
            .context("Failed to update an user in DB")?;
        Ok(())
    }

    pub(crate) async fn delete_user(&self, id: &UserId) -> Result<(), Failure> {
        let query = formatdoc! {r"
            DELETE FROM `{TABLE_USERS}`
            WHERE `id` = ?
        "};
        sqlx::query(&query)
            .bind(id.0)
            .execute(&self.0)
            .await
            .context("Failed to delete an user from DB")?;
        Ok(())
    }
}
