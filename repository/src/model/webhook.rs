use std::iter;

use async_trait::async_trait;
use indoc::{formatdoc, indoc};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlRow, FromRow, Result};
use uuid::Uuid;

use crate::{parse_col_str2uuid, DatabaseImpl};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Webhook {
    pub id: Uuid,
    pub channel_id: Uuid,
    pub owner_id: Uuid,
}

impl<'r> FromRow<'r, MySqlRow> for Webhook {
    fn from_row(row: &'r MySqlRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(Self {
            id: parse_col_str2uuid(row, "id")?,
            channel_id: parse_col_str2uuid(row, "channel_id")?,
            owner_id: parse_col_str2uuid(row, "owner_id")?,
        })
    }
}

#[async_trait]
pub trait WebhookDb {
    async fn read(&self) -> Result<Vec<Webhook>>;
    async fn find(&self, id: &Uuid) -> Result<Option<Webhook>>;
    async fn filter_by_cid(&self, channel_id: Uuid) -> Result<Vec<Webhook>>;
    async fn filter_by_oid(&self, owner_id: Uuid) -> Result<Vec<Webhook>>;
    async fn filter_by_cids(&self, cids: &[Uuid]) -> Result<Vec<Webhook>>;
    async fn filter_by_oids(&self, oids: &[Uuid]) -> Result<Vec<Webhook>>;
    async fn create(&self, w: Webhook) -> Result<()>;
    async fn create_ignore(&self, ws: &[Webhook]) -> Result<()>;
    async fn update(&self, id: &str, w: Webhook) -> Result<()>;
    async fn delete(&self, id: &Uuid) -> Result<()>;
}

#[async_trait]
impl WebhookDb for DatabaseImpl {
    async fn read(&self) -> Result<Vec<Webhook>> {
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

    async fn find(&self, id: &Uuid) -> Result<Option<Webhook>> {
        sqlx::query(indoc! {r#"
            SELECT *
            FROM `webhooks`
            WHERE `id` = ?
            LIMIT 1
        "#})
        .bind(id.to_string())
        .fetch_optional(&self.0)
        .await?
        .map(|w| Webhook::from_row(&w))
        .transpose()
    }

    async fn filter_by_cid(&self, channel_id: Uuid) -> Result<Vec<Webhook>> {
        sqlx::query(indoc! {r#"
            SELECT *
            FROM `webhooks`
            WHERE `channel_id` = ?
        "#})
        .bind(channel_id.to_string())
        .fetch_all(&self.0)
        .await?
        .iter()
        .map(Webhook::from_row)
        .collect()
    }

    async fn filter_by_oid(&self, owner_id: Uuid) -> Result<Vec<Webhook>> {
        sqlx::query(indoc! {r#"
            SELECT *
            FROM `webhooks`
            WHERE `owner_id` = ?
        "#})
        .bind(owner_id.to_string())
        .fetch_all(&self.0)
        .await?
        .iter()
        .map(Webhook::from_row)
        .collect()
    }

    async fn filter_by_cids(&self, cids: &[Uuid]) -> Result<Vec<Webhook>> {
        let cid_len = cids.len();
        if cid_len == 0 {
            return Ok(vec![]);
        }
        let query = formatdoc! {r#"
                SELECT *
                FROM `webhooks`
                WHERE `channel_id` IN ({})
            "#,
            iter::repeat('?').take(cid_len).join(", ")
        };
        let query = cids.iter().fold(sqlx::query(&query), |q, i| q.bind(i));
        query
            .fetch_all(&self.0)
            .await?
            .iter()
            .map(Webhook::from_row)
            .collect()
    }

    async fn filter_by_oids(&self, oids: &[Uuid]) -> Result<Vec<Webhook>> {
        let oid_len = oids.len();
        if oid_len == 0 {
            return Ok(vec![]);
        }
        let query = formatdoc! {r#"
                SELECT *
                FROM `webhooks`
                WHERE `owner_id` IN ({})
            "#,
            iter::repeat('?').take(oid_len).join(", ")
        };
        let query = oids
            .iter()
            .fold(sqlx::query(&query), |q, i| q.bind(i.to_string()));
        query
            .fetch_all(&self.0)
            .await?
            .iter()
            .map(Webhook::from_row)
            .collect()
    }

    async fn create(&self, w: Webhook) -> Result<()> {
        sqlx::query(indoc! {r#"
            INSERT INTO `webhooks` (`id`, `channel_id`, `owner_id`)
            VALUES (?, ?, ?)
        "#})
        .bind(w.id.to_string())
        .bind(w.channel_id.to_string())
        .bind(w.owner_id.to_string())
        .execute(&self.0)
        .await?;
        Ok(())
    }

    async fn create_ignore(&self, ws: &[Webhook]) -> Result<()> {
        let ws_len = ws.len();
        if ws_len == 0 {
            return Ok(());
        }
        let query = formatdoc! {
            r#"
                INSERT IGNORE
                INTO `webhooks` (`id`, `channel_id`, `owner_id`)
                VALUES {}
            "#,
            iter::repeat("(?, ?, ?)").take(ws_len).join(", ")
        };
        let query = ws.iter().fold(sqlx::query(&query), |q, w| {
            q.bind(w.id.to_string())
                .bind(w.channel_id.to_string())
                .bind(w.owner_id.to_string())
        });
        query.execute(&self.0).await?;
        Ok(())
    }

    async fn update(&self, id: &str, w: Webhook) -> Result<()> {
        sqlx::query(indoc! {r#"
            UPDATE `users`
            SET `id` = ?, `channel_id` = ?, `owner_id` = ?
            WHERE `id` = ?
        "#})
        .bind(w.id.to_string())
        .bind(w.channel_id.to_string())
        .bind(w.owner_id.to_string())
        .bind(id)
        .execute(&self.0)
        .await?;
        Ok(())
    }

    async fn delete(&self, id: &Uuid) -> Result<()> {
        sqlx::query(indoc! {r#"
            DELETE FROM `webhooks`
            WHERE `id` = ?
        "#})
        .bind(id.to_string())
        .execute(&self.0)
        .await?;
        Ok(())
    }
}
