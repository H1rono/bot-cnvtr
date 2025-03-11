use std::iter;

use anyhow::Context;
use indoc::formatdoc;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, mysql::MySqlRow};
use uuid::Uuid;

use domain::{ChannelId, Failure, OwnerId, WebhookId};

use crate::RepositoryImpl;

const TABLE_WEBHOOKS: &str = "webhooks_v2";

#[must_use]
#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
struct WebhookRow {
    pub id: Uuid,
    pub channel_id: Uuid,
    pub owner_id: Uuid,
}

#[must_use]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Webhook {
    pub id: WebhookId,
    pub channel_id: ChannelId,
    pub owner_id: OwnerId,
}

impl From<WebhookRow> for Webhook {
    fn from(value: WebhookRow) -> Self {
        let WebhookRow {
            id,
            channel_id,
            owner_id,
        } = value;
        Self {
            id: id.into(),
            channel_id: channel_id.into(),
            owner_id: owner_id.into(),
        }
    }
}

impl<'r> FromRow<'r, MySqlRow> for Webhook {
    fn from_row(row: &'r MySqlRow) -> sqlx::Result<Self> {
        WebhookRow::from_row(row).map(Self::from)
    }
}

#[allow(dead_code)]
impl RepositoryImpl {
    pub(crate) async fn read_webhooks(&self) -> Result<Vec<Webhook>, Failure> {
        let query = formatdoc! {r"
            SELECT *
            FROM `{TABLE_WEBHOOKS}`
        "};
        let res = sqlx::query_as(&query)
            .fetch_all(&self.0)
            .await
            .context("Failed to read webhooks from DB")?;
        Ok(res)
    }

    pub(crate) async fn find_webhook(&self, id: &WebhookId) -> Result<Webhook, Failure> {
        let query = formatdoc! {r"
            SELECT *
            FROM `{TABLE_WEBHOOKS}`
            WHERE `id` = ?
            LIMIT 1
        "};
        sqlx::query_as(&query)
            .bind(id.0)
            .fetch_optional(&self.0)
            .await
            .context("Failed to read a webhook from DB")?
            .ok_or_else(|| Failure::reject_not_found("No webhook found"))
    }

    pub(crate) async fn filter_webhooks_by_cid(
        &self,
        channel_id: ChannelId,
    ) -> Result<Vec<Webhook>, Failure> {
        let query = formatdoc! {r"
            SELECT *
            FROM `{TABLE_WEBHOOKS}`
            WHERE `channel_id` = ?
        "};
        let res = sqlx::query_as(&query)
            .bind(channel_id.0)
            .fetch_all(&self.0)
            .await
            .context("Failed to read-filter webhooks from DB")?;
        Ok(res)
    }

    pub(crate) async fn filter_webhooks_by_oid(
        &self,
        owner_id: OwnerId,
    ) -> Result<Vec<Webhook>, Failure> {
        let query = formatdoc! {r"
            SELECT *
            FROM `{TABLE_WEBHOOKS}`
            WHERE `owner_id` = ?
        "};
        let res = sqlx::query_as(&query)
            .bind(owner_id.0)
            .fetch_all(&self.0)
            .await
            .context("Failed to read-filter webhooks from DB")?;
        Ok(res)
    }

    pub(crate) async fn filter_webhooks_by_cids(
        &self,
        cids: &[ChannelId],
    ) -> Result<Vec<Webhook>, Failure> {
        if cids.is_empty() {
            return Ok(vec![]);
        }
        let ids_arg = iter::repeat('?').take(cids.len()).join(", ");
        let query = formatdoc! {r"
            SELECT *
            FROM `{TABLE_WEBHOOKS}`
            WHERE `channel_id` IN ({ids_arg})
        "};
        let res = cids
            .iter()
            .map(ToString::to_string)
            .fold(sqlx::query_as(&query), sqlx::query::QueryAs::bind)
            .fetch_all(&self.0)
            .await
            .context("Failed to read-filter webhooks from DB")?;
        Ok(res)
    }

    pub(crate) async fn filter_webhooks_by_oids(
        &self,
        oids: &[OwnerId],
    ) -> Result<Vec<Webhook>, Failure> {
        if oids.is_empty() {
            return Ok(vec![]);
        }
        let ids_arg = iter::repeat('?').take(oids.len()).join(", ");
        let query = formatdoc! {r"
            SELECT *
            FROM `{TABLE_WEBHOOKS}`
            WHERE `owner_id` IN ({ids_arg})
        "};
        let res = oids
            .iter()
            .fold(sqlx::query_as(&query), |q, i| q.bind(i.0))
            .fetch_all(&self.0)
            .await
            .context("Failed to read-filter webhooks from DB")?;
        Ok(res)
    }

    pub(crate) async fn create_webhook(&self, w: Webhook) -> Result<(), Failure> {
        let query = formatdoc! {r"
            INSERT INTO `{TABLE_WEBHOOKS}` (`id`, `channel_id`, `owner_id`)
            VALUES (?, ?, ?)
        "};
        sqlx::query(&query)
            .bind(w.id.0)
            .bind(w.channel_id.0)
            .bind(w.owner_id.0)
            .execute(&self.0)
            .await
            .context("Failed to create webhook to DB")?;
        Ok(())
    }

    pub(crate) async fn create_ignore_webhooks(&self, ws: &[Webhook]) -> Result<(), Failure> {
        if ws.is_empty() {
            return Ok(());
        }
        let values_arg = iter::repeat("(?, ?, ?)").take(ws.len()).join(", ");
        let query = formatdoc! {r"
            INSERT IGNORE
            INTO `{TABLE_WEBHOOKS}` (`id`, `channel_id`, `owner_id`)
            VALUES {values_arg}
        "};
        let query = ws.iter().fold(sqlx::query(&query), |q, w| {
            q.bind(w.id.0).bind(w.channel_id.0).bind(w.owner_id.0)
        });
        query
            .execute(&self.0)
            .await
            .context("Failed to create webhooks to DB")?;
        Ok(())
    }

    pub(crate) async fn update_webhook(&self, id: &WebhookId, w: Webhook) -> Result<(), Failure> {
        let query = formatdoc! {r"
            UPDATE `{TABLE_WEBHOOKS}`
            SET `id` = ?, `channel_id` = ?, `owner_id` = ?
            WHERE `id` = ?
        "};
        sqlx::query(&query)
            .bind(w.id.0)
            .bind(w.channel_id.0)
            .bind(w.owner_id.0)
            .bind(id.0)
            .execute(&self.0)
            .await
            .context("Failed to update webhook in DB")?;
        Ok(())
    }

    pub(crate) async fn delete_webhook(&self, id: &WebhookId) -> Result<(), Failure> {
        let query = formatdoc! {r"
            DELETE FROM `{TABLE_WEBHOOKS}`
            WHERE `id` = ?
        "};
        sqlx::query(&query)
            .bind(id.0)
            .execute(&self.0)
            .await
            .context("Failed to delete webhook from DB")?;
        Ok(())
    }
}
