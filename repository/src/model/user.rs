use std::iter;

use async_trait::async_trait;
use indoc::{formatdoc, indoc};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlRow, FromRow, Result, Row};
use uuid::Uuid;

use crate::{parse_col_str2uuid, DatabaseImpl};

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

#[async_trait]
pub trait UserRepository {
    async fn read(&self) -> Result<Vec<User>>;
    async fn find(&self, id: &Uuid) -> Result<Option<User>>;
    async fn create(&self, u: User) -> Result<()>;
    async fn create_ignore(&self, us: &[User]) -> Result<()>;
    async fn update(&self, id: &Uuid, u: User) -> Result<()>;
    async fn delete(&self, id: &Uuid) -> Result<()>;
}

#[async_trait]
impl UserRepository for DatabaseImpl {
    async fn read(&self) -> Result<Vec<User>> {
        sqlx::query_as(indoc! {r#"
            SELECT *
            FROM `users`
        "#})
        .fetch_all(&self.0)
        .await
    }

    async fn find(&self, id: &Uuid) -> Result<Option<User>> {
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

    async fn create(&self, u: User) -> Result<()> {
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

    async fn create_ignore(&self, us: &[User]) -> Result<()> {
        let us_len = us.len();
        if us_len == 0 {
            return Ok(());
        }
        let query = formatdoc! {
            r#"
                INSERT IGNORE
                INTO `users` (`id`, `name`)
                VALUES {}
            "#,
            iter::repeat("(?, ?)").take(us_len).join(", ")
        };
        let query = us.iter().fold(sqlx::query(&query), |q, u| {
            q.bind(u.id.to_string()).bind(&u.name)
        });
        query.execute(&self.0).await?;
        Ok(())
    }

    async fn update(&self, id: &Uuid, u: User) -> Result<()> {
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

    async fn delete(&self, id: &Uuid) -> Result<()> {
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
