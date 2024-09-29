use std::iter;

use domain::{ChannelId, OwnerId, WebhookId};
use indoc::formatdoc;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlRow, FromRow};

use super::parse_col_str2uuid;
use crate::error::{Error, Result};
use crate::RepositoryImpl;

const TABLE_WEBHOOKS: &str = "webhooks";

#[must_use]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Webhook {
    pub id: WebhookId,
    pub channel_id: ChannelId,
    pub owner_id: OwnerId,
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
        let query = formatdoc! {r"
            SELECT *
            FROM `{TABLE_WEBHOOKS}`
        "};
        sqlx::query_as(&query)
            .fetch_all(&self.0)
            .await
            .map_err(Error::from)
    }

    pub(crate) async fn find_webhook(&self, id: &WebhookId) -> Result<Option<Webhook>> {
        let query = formatdoc! {r"
            SELECT *
            FROM `{TABLE_WEBHOOKS}`
            WHERE `id` = ?
            LIMIT 1
        "};
        sqlx::query_as(&query)
            .bind(id.to_string())
            .fetch_optional(&self.0)
            .await
            .map_err(Error::from)
    }

    pub(crate) async fn filter_webhooks_by_cid(
        &self,
        channel_id: ChannelId,
    ) -> Result<Vec<Webhook>> {
        let query = formatdoc! {r"
            SELECT *
            FROM `{TABLE_WEBHOOKS}`
            WHERE `channel_id` = ?
        "};
        sqlx::query_as(&query)
            .bind(channel_id.to_string())
            .fetch_all(&self.0)
            .await
            .map_err(Error::from)
    }

    pub(crate) async fn filter_webhooks_by_oid(&self, owner_id: OwnerId) -> Result<Vec<Webhook>> {
        let query = formatdoc! {r"
            SELECT *
            FROM `{TABLE_WEBHOOKS}`
            WHERE `owner_id` = ?
        "};
        sqlx::query_as(&query)
            .bind(owner_id.to_string())
            .fetch_all(&self.0)
            .await
            .map_err(Error::from)
    }

    pub(crate) async fn filter_webhooks_by_cids(&self, cids: &[ChannelId]) -> Result<Vec<Webhook>> {
        let cid_len = cids.len();
        if cid_len == 0 {
            return Ok(vec![]);
        }
        let ids_arg = iter::repeat('?').take(cid_len).join(", ");
        let query = formatdoc! {r"
            SELECT *
            FROM `{TABLE_WEBHOOKS}`
            WHERE `channel_id` IN ({ids_arg})
        "};
        cids.iter()
            .map(ToString::to_string)
            .fold(sqlx::query_as(&query), sqlx::query::QueryAs::bind)
            .fetch_all(&self.0)
            .await
            .map_err(Error::from)
    }

    pub(crate) async fn filter_webhooks_by_oids(&self, oids: &[OwnerId]) -> Result<Vec<Webhook>> {
        let oid_len = oids.len();
        if oid_len == 0 {
            return Ok(vec![]);
        }
        let ids_arg = iter::repeat('?').take(oid_len).join(", ");
        let query = formatdoc! {r"
            SELECT *
            FROM `{TABLE_WEBHOOKS}`
            WHERE `owner_id` IN ({ids_arg})
        "};
        oids.iter()
            .fold(sqlx::query_as(&query), |q, i| q.bind(i.to_string()))
            .fetch_all(&self.0)
            .await
            .map_err(Error::from)
    }

    pub(crate) async fn create_webhook(&self, w: Webhook) -> Result<()> {
        let query = formatdoc! {r"
            INSERT INTO `{TABLE_WEBHOOKS}` (`id`, `channel_id`, `owner_id`)
            VALUES (?, ?, ?)
        "};
        sqlx::query(&query)
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
            INTO `{TABLE_WEBHOOKS}` (`id`, `channel_id`, `owner_id`)
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

    pub(crate) async fn update_webhook(&self, id: &WebhookId, w: Webhook) -> Result<()> {
        let query = formatdoc! {r"
            UPDATE `{TABLE_WEBHOOKS}`
            SET `id` = ?, `channel_id` = ?, `owner_id` = ?
            WHERE `id` = ?
        "};
        sqlx::query(&query)
            .bind(w.id.to_string())
            .bind(w.channel_id.to_string())
            .bind(w.owner_id.to_string())
            .bind(id.to_string())
            .execute(&self.0)
            .await?;
        Ok(())
    }

    pub(crate) async fn delete_webhook(&self, id: &WebhookId) -> Result<()> {
        let query = formatdoc! {r"
            DELETE FROM `{TABLE_WEBHOOKS}`
            WHERE `id` = ?
        "};
        sqlx::query(&query)
            .bind(id.to_string())
            .execute(&self.0)
            .await?;
        Ok(())
    }
}
