use std::iter;

use indoc::{formatdoc, indoc};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlRow, FromRow, Result};
use uuid::Uuid;

use super::parse_col_str2uuid;
use crate::RepositoryImpl;

#[must_use]
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

#[allow(dead_code)]
impl RepositoryImpl {
    pub(crate) async fn read_webhooks(&self) -> Result<Vec<Webhook>> {
        sqlx::query_as(indoc! {r"
            SELECT *
            FROM `webhooks`
        "})
        .fetch_all(&self.0)
        .await
    }

    pub(crate) async fn find_webhook(&self, id: &Uuid) -> Result<Option<Webhook>> {
        sqlx::query_as(indoc! {r"
            SELECT *
            FROM `webhooks`
            WHERE `id` = ?
            LIMIT 1
        "})
        .bind(id.to_string())
        .fetch_optional(&self.0)
        .await
    }

    pub(crate) async fn filter_webhooks_by_cid(&self, channel_id: Uuid) -> Result<Vec<Webhook>> {
        sqlx::query_as(indoc! {r"
            SELECT *
            FROM `webhooks`
            WHERE `channel_id` = ?
        "})
        .bind(channel_id.to_string())
        .fetch_all(&self.0)
        .await
    }

    pub(crate) async fn filter_webhooks_by_oid(&self, owner_id: Uuid) -> Result<Vec<Webhook>> {
        sqlx::query_as(indoc! {r"
            SELECT *
            FROM `webhooks`
            WHERE `owner_id` = ?
        "})
        .bind(owner_id.to_string())
        .fetch_all(&self.0)
        .await
    }

    pub(crate) async fn filter_webhooks_by_cids(&self, cids: &[Uuid]) -> Result<Vec<Webhook>> {
        let cid_len = cids.len();
        if cid_len == 0 {
            return Ok(vec![]);
        }
        let ids_arg = iter::repeat('?').take(cid_len).join(", ");
        let query = formatdoc! {r"
            SELECT *
            FROM `webhooks`
            WHERE `channel_id` IN ({ids_arg})
        "};
        cids.iter()
            .fold(sqlx::query_as(&query), sqlx::query::QueryAs::bind)
            .fetch_all(&self.0)
            .await
    }

    pub(crate) async fn filter_webhooks_by_oids(&self, oids: &[Uuid]) -> Result<Vec<Webhook>> {
        let oid_len = oids.len();
        if oid_len == 0 {
            return Ok(vec![]);
        }
        let ids_arg = iter::repeat('?').take(oid_len).join(", ");
        let query = formatdoc! {r"
            SELECT *
            FROM `webhooks`
            WHERE `owner_id` IN ({ids_arg})
        "};
        oids.iter()
            .fold(sqlx::query_as(&query), |q, i| q.bind(i.to_string()))
            .fetch_all(&self.0)
            .await
    }

    pub(crate) async fn create_webhook(&self, w: Webhook) -> Result<()> {
        sqlx::query(indoc! {r"
            INSERT INTO `webhooks` (`id`, `channel_id`, `owner_id`)
            VALUES (?, ?, ?)
        "})
        .bind(w.id.to_string())
        .bind(w.channel_id.to_string())
        .bind(w.owner_id.to_string())
        .execute(&self.0)
        .await?;
        Ok(())
    }

    pub(crate) async fn create_ignore_webhooks(&self, ws: &[Webhook]) -> Result<()> {
        if ws.is_empty() {
            return Ok(());
        }
        let values_arg = iter::repeat("(?, ?, ?)").take(ws.len()).join(", ");
        let query = formatdoc! {r"
            INSERT IGNORE
            INTO `webhooks` (`id`, `channel_id`, `owner_id`)
            VALUES {values_arg}
        "};
        let query = ws.iter().fold(sqlx::query(&query), |q, w| {
            q.bind(w.id.to_string())
                .bind(w.channel_id.to_string())
                .bind(w.owner_id.to_string())
        });
        query.execute(&self.0).await?;
        Ok(())
    }

    pub(crate) async fn update_webhook(&self, id: &Uuid, w: Webhook) -> Result<()> {
        sqlx::query(indoc! {r"
            UPDATE `users`
            SET `id` = ?, `channel_id` = ?, `owner_id` = ?
            WHERE `id` = ?
        "})
        .bind(w.id.to_string())
        .bind(w.channel_id.to_string())
        .bind(w.owner_id.to_string())
        .bind(id)
        .execute(&self.0)
        .await?;
        Ok(())
    }

    pub(crate) async fn delete_webhook(&self, id: &Uuid) -> Result<()> {
        sqlx::query(indoc! {r"
            DELETE FROM `webhooks`
            WHERE `id` = ?
        "})
        .bind(id.to_string())
        .execute(&self.0)
        .await?;
        Ok(())
    }
}
