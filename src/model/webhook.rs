use std::iter;

use indoc::{formatdoc, indoc};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlRow, FromRow, Result, Row};
use uuid::Uuid;

use super::{parse_col_str2uuid, Database};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Webhook {
    pub id: String,
    pub channel_id: Uuid,
    pub owner_id: Uuid,
}

impl<'r> FromRow<'r, MySqlRow> for Webhook {
    fn from_row(row: &'r MySqlRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            channel_id: parse_col_str2uuid(row, "channel_id")?,
            owner_id: parse_col_str2uuid(row, "owner_id")?,
        })
    }
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
        .bind(channel_id.to_string())
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
        .bind(owner_id.to_string())
        .fetch_all(&self.0)
        .await?
        .iter()
        .map(Webhook::from_row)
        .collect()
    }

    pub async fn filter_webhooks_by_cids(&self, cids: &[Uuid]) -> Result<Vec<Webhook>> {
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

    pub async fn filter_webhooks_by_oids(&self, oids: &[Uuid]) -> Result<Vec<Webhook>> {
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

    pub async fn create_webhook(&self, w: Webhook) -> Result<()> {
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

    pub async fn create_ignore_webhooks(&self, ws: &[Webhook]) -> Result<()> {
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
            q.bind(&w.id)
                .bind(w.channel_id.to_string())
                .bind(w.owner_id.to_string())
        });
        query.execute(&self.0).await?;
        Ok(())
    }

    pub async fn update_webhook(&self, id: &str, w: Webhook) -> Result<()> {
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
