use std::iter;

use async_trait::async_trait;
use indoc::{formatdoc, indoc};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlRow, FromRow, Result, Row};
use uuid::Uuid;

use crate::{parse_col_str2uuid, DatabaseImpl};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Owner {
    pub id: Uuid,
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

#[async_trait]
pub trait OwnerRepository {
    async fn read(&self) -> Result<Vec<Owner>>;
    async fn find(&self, id: &Uuid) -> Result<Option<Owner>>;
    async fn create(&self, o: Owner) -> Result<()>;
    async fn create_ignore(&self, os: &[Owner]) -> Result<()>;
    async fn update(&self, id: &Uuid, o: Owner) -> Result<()>;
    async fn delete(&self, id: &Uuid) -> Result<()>;
}

#[async_trait]
impl OwnerRepository for DatabaseImpl {
    async fn read(&self) -> Result<Vec<Owner>> {
        sqlx::query_as(indoc! {r#"
            SELECT *
            FROM `owners`
        "#})
        .fetch_all(&self.0)
        .await
    }

    async fn find(&self, id: &Uuid) -> Result<Option<Owner>> {
        sqlx::query_as(indoc! {r#"
            SELECT *
            FROM `owners`
            WHERE `id` = ?
            LIMIT 1
        "#})
        .bind(id.to_string())
        .fetch_optional(&self.0)
        .await
    }

    async fn create(&self, o: Owner) -> Result<()> {
        sqlx::query(indoc! {r#"
            INSERT INTO `owners` (`id`, `name`, `group`)
            VALUES (?, ?, ?)
        "#})
        .bind(o.id.to_string())
        .bind(o.name)
        .bind(o.group)
        .execute(&self.0)
        .await?;
        Ok(())
    }

    async fn create_ignore(&self, os: &[Owner]) -> Result<()> {
        let os_len = os.len();
        if os_len == 0 {
            return Ok(());
        }
        let query = formatdoc! {
            r#"
                INSERT IGNORE
                INTO `owners` (`id`, `name`, `group`)
                VALUES {}
            "#,
            iter::repeat("(?, ?, ?)").take(os_len).join(", ")
        };
        let query = os.iter().fold(sqlx::query(&query), |q, o| {
            q.bind(o.id.to_string()).bind(&o.name).bind(o.group)
        });
        query.execute(&self.0).await?;
        Ok(())
    }

    async fn update(&self, id: &Uuid, o: Owner) -> Result<()> {
        sqlx::query(indoc! {r#"
            UPDATE `owners`
            SET `id` = ?, `name` = ?, `group` = ?
            WHERE `id` = ?
        "#})
        .bind(o.id.to_string())
        .bind(o.name)
        .bind(o.group)
        .bind(id.to_string())
        .execute(&self.0)
        .await?;
        Ok(())
    }

    async fn delete(&self, id: &Uuid) -> Result<()> {
        sqlx::query(indoc! {r#"
            DELETE FROM `owners`
            WHERE `id` = ?
        "#})
        .bind(id.to_string())
        .execute(&self.0)
        .await?;
        Ok(())
    }
}
