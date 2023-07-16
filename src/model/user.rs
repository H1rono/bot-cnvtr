use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Result};

use super::Database;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, FromRow)]
pub struct User {
    pub id: String,
    pub name: String,
}

impl Database {
    pub async fn read_users(&self) -> Result<Vec<User>> {
        sqlx::query(r#"SELECT * FROM `users`"#)
            .fetch_all(&self.0)
            .await?
            .into_iter()
            .map(|u| User::from_row(&u))
            .collect::<Result<_>>()
    }

    pub async fn create_user(&self, u: User) -> Result<()> {
        sqlx::query(r#"INSERT INTO `users` (`id`, `name`) VALUES (?, ?)"#)
            .bind(u.id)
            .bind(u.name)
            .execute(&self.0)
            .await?;
        Ok(())
    }

    pub async fn update_user(&self, id: &str, u: User) -> Result<()> {
        sqlx::query(r#"UPDATE `users` SET `id` = ?, `name` = ? WHERE `id` = ?"#)
            .bind(u.id)
            .bind(u.name)
            .bind(id)
            .execute(&self.0)
            .await?;
        Ok(())
    }

    pub async fn delete_user(&self, id: &str) -> Result<()> {
        sqlx::query(r#"DELETE FROM `users` WHERE `id` = ?"#)
            .bind(id)
            .execute(&self.0)
            .await?;
        Ok(())
    }
}
