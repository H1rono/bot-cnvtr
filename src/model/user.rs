use indoc::indoc;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Result};
use uuid::Uuid;

use super::Database;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub name: String,
}

impl Database {
    pub async fn read_users(&self) -> Result<Vec<User>> {
        sqlx::query(indoc! {r#"
            SELECT *
            FROM `users`
        "#})
        .fetch_all(&self.0)
        .await?
        .iter()
        .map(User::from_row)
        .collect::<Result<_>>()
    }

    pub async fn find_user(&self, id: &Uuid) -> Result<Option<User>> {
        sqlx::query(indoc! {r#"
            SELECT *
            FROM `users`
            WHERE `id` = ?
            LIMIT 1
        "#})
        .bind(id)
        .fetch_optional(&self.0)
        .await?
        .map(|u| User::from_row(&u))
        .transpose()
    }

    pub async fn create_user(&self, u: User) -> Result<()> {
        sqlx::query(indoc! {r#"
            INSERT INTO `users` (`id`, `name`)
            VALUES (?, ?)
        "#})
        .bind(u.id)
        .bind(u.name)
        .execute(&self.0)
        .await?;
        Ok(())
    }

    pub async fn update_user(&self, id: &Uuid, u: User) -> Result<()> {
        sqlx::query(indoc! {r#"
            UPDATE `users`
            SET `id` = ?, `name` = ?
            WHERE `id` = ?
        "#})
        .bind(u.id)
        .bind(u.name)
        .bind(id)
        .execute(&self.0)
        .await?;
        Ok(())
    }

    pub async fn delete_user(&self, id: &Uuid) -> Result<()> {
        sqlx::query(indoc! {r#"
            DELETE FROM `users`
            WHERE `id` = ?
        "#})
        .bind(id)
        .execute(&self.0)
        .await?;
        Ok(())
    }
}
