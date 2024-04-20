use std::iter;

use indoc::{formatdoc, indoc};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlRow, FromRow, Result, Row};
use uuid::Uuid;

use super::parse_col_str2uuid;
use crate::RepositoryImpl;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct User {
    pub id: Uuid,
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
        sqlx::query_as(indoc! {r#"
            SELECT *
            FROM `users`
        "#})
        .fetch_all(&self.0)
        .await
    }

    pub(crate) async fn find_user(&self, id: &Uuid) -> Result<Option<User>> {
        sqlx::query_as(indoc! {r#"
            SELECT *
            FROM `users`
            WHERE `id` = ?
            LIMIT 1
        "#})
        .bind(id.to_string())
        .fetch_optional(&self.0)
        .await
    }

    pub(crate) async fn create_user(&self, u: User) -> Result<()> {
        sqlx::query(indoc! {r#"
            INSERT INTO `users` (`id`, `name`)
            VALUES (?, ?)
        "#})
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
        let query = formatdoc! {
            r#"
                INSERT IGNORE
                INTO `users` (`id`, `name`)
                VALUES {}
            "#,
            iter::repeat("(?, ?)").take(us.len()).join(", ")
        };
        let query = us.iter().fold(sqlx::query(&query), |q, u| {
            q.bind(u.id.to_string()).bind(&u.name)
        });
        query.execute(&self.0).await?;
        Ok(())
    }

    pub(crate) async fn update_user(&self, id: &Uuid, u: User) -> Result<()> {
        sqlx::query(indoc! {r#"
            UPDATE `users`
            SET `id` = ?, `name` = ?
            WHERE `id` = ?
        "#})
        .bind(u.id.to_string())
        .bind(u.name)
        .bind(id.to_string())
        .execute(&self.0)
        .await?;
        Ok(())
    }

    pub(crate) async fn delete_user(&self, id: &Uuid) -> Result<()> {
        sqlx::query(indoc! {r#"
            DELETE FROM `users`
            WHERE `id` = ?
        "#})
        .bind(id.to_string())
        .execute(&self.0)
        .await?;
        Ok(())
    }
}
