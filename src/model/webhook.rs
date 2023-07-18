use indoc::indoc;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Result};
use uuid::Uuid;

use super::Database;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, FromRow)]
pub struct Webhook {
    pub id: String,
    pub channel_id: Uuid,
    pub owner_id: Uuid,
}

impl Database {
    pub async fn read_webhooks(&self) -> Result<Vec<Webhook>> {
        sqlx::query(indoc! {r#"
            SELECT *
            FROM `webhooks`
        "#})
        .fetch_all(&self.0)
        .await?
        .iter()
        .map(Webhook::from_row)
        .collect::<Result<_>>()
    }

    pub async fn find_webhook(&self, id: &str) -> Result<Option<Webhook>> {
        sqlx::query(indoc! {r#"
            SELECT *
            FROM `webhooks`
            WHERE `id` = ?
            LIMIT 1
        "#})
        .bind(id)
        .fetch_optional(&self.0)
        .await?
        .map(|w| Webhook::from_row(&w))
        .transpose()
    }

    pub async fn filter_webhooks_by_cid(&self, channel_id: Uuid) -> Result<Vec<Webhook>> {
        sqlx::query(indoc! {r#"
            SELECT *
            FROM `webhooks`
            WHERE `channel_id` = ?
        "#})
        .bind(channel_id)
        .fetch_all(&self.0)
        .await?
        .iter()
        .map(Webhook::from_row)
        .collect()
    }

    pub async fn filter_webhooks_by_oid(&self, owner_id: Uuid) -> Result<Vec<Webhook>> {
        sqlx::query(indoc! {r#"
            SELECT *
            FROM `webhooks`
            WHERE `owner_id` = ?
        "#})
        .bind(owner_id)
        .fetch_all(&self.0)
        .await?
        .iter()
        .map(Webhook::from_row)
        .collect()
    }

    pub async fn create_webhook(&self, w: Webhook) -> Result<()> {
        sqlx::query(indoc! {r#"
            INSERT INTO `webhooks` (`id`, `channel_id`, `owner_id`)
            VALUES (?, ?, ?)
        "#})
        .bind(w.id)
        .bind(w.channel_id)
        .bind(w.owner_id)
        .execute(&self.0)
        .await?;
        Ok(())
    }

    pub async fn update_webhook(&self, id: &str, w: Webhook) -> Result<()> {
        sqlx::query(indoc! {r#"
            UPDATE `users`
            SET `id` = ?, `channel_id` = ?, `owner_id` = ?
            WHERE `id` = ?
        "#})
        .bind(w.id)
        .bind(w.channel_id)
        .bind(w.owner_id)
        .bind(id)
        .execute(&self.0)
        .await?;
        Ok(())
    }

    pub async fn delete_webhook(&self, id: &str) -> Result<()> {
        sqlx::query(indoc! {r#"
            DELETE FROM `webhooks`
            WHERE `id` = ?
        "#})
        .bind(id)
        .execute(&self.0)
        .await?;
        Ok(())
    }
}
