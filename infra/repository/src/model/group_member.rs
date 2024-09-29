use std::iter;

use indoc::formatdoc;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlRow, FromRow};

use domain::{GroupId, UserId};

use super::parse_col_str2uuid;
use crate::error::{Error, Result};
use crate::RepositoryImpl;

const TABLE_GROUP_MEMBERS: &str = "group_members";

#[must_use]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct GroupMember {
    pub group_id: GroupId,
    pub user_id: UserId,
}

impl<'r> FromRow<'r, MySqlRow> for GroupMember {
    fn from_row(row: &'r MySqlRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(Self {
            group_id: parse_col_str2uuid(row, "group_id")?,
            user_id: parse_col_str2uuid(row, "user_id")?,
        })
    }
}

#[allow(dead_code)]
impl RepositoryImpl {
    pub(crate) async fn read_group_members(&self) -> Result<Vec<GroupMember>> {
        let query = formatdoc! {r"
            SELECT *
            FROM `{TABLE_GROUP_MEMBERS}`
        "};
        sqlx::query_as(&query)
            .fetch_all(&self.0)
            .await
            .map_err(Error::from)
    }

    pub(crate) async fn find_group_member(
        &self,
        gid: &GroupId,
        uid: &UserId,
    ) -> Result<Option<GroupMember>> {
        let query = formatdoc! {r"
            SELECT *
            FROM `{TABLE_GROUP_MEMBERS}`
            WHERE `group_id` = ?, `user_id` = ?
            LIMIT 1
        "};
        sqlx::query_as(&query)
            .bind(gid.to_string())
            .bind(uid.to_string())
            .fetch_optional(&self.0)
            .await
            .map_err(Error::from)
    }

    pub(crate) async fn filter_group_members_by_gid(
        &self,
        gid: &GroupId,
    ) -> Result<Vec<GroupMember>> {
        let query = formatdoc! {r"
            SELECT *
            FROM `{TABLE_GROUP_MEMBERS}`
            WHERE `group_id` = ?
        "};
        sqlx::query_as(&query)
            .bind(gid.to_string())
            .fetch_all(&self.0)
            .await
            .map_err(Error::from)
    }

    pub(crate) async fn filter_group_members_by_uid(
        &self,
        uid: &UserId,
    ) -> Result<Vec<GroupMember>> {
        let query = formatdoc! {r"
            SELECT *
            FROM `{TABLE_GROUP_MEMBERS}`
            WHERE `user_id` = ?
        "};
        sqlx::query_as(&query)
            .bind(uid.to_string())
            .fetch_all(&self.0)
            .await
            .map_err(Error::from)
    }

    pub(crate) async fn create_group_member(&self, gm: GroupMember) -> Result<()> {
        let query = formatdoc! {r"
            INSERT
            INTO `{TABLE_GROUP_MEMBERS}` (`group_id`, `user_id`)
            VALUES (?, ?)
        "};
        sqlx::query(&query)
            .bind(gm.group_id.to_string())
            .bind(gm.user_id.to_string())
            .execute(&self.0)
            .await?;
        Ok(())
    }

    pub(crate) async fn create_ignore_group_members(&self, gms: &[GroupMember]) -> Result<()> {
        if gms.is_empty() {
            return Ok(());
        }
        let values_arg = iter::repeat("(?, ?)").take(gms.len()).join(", ");
        let query = formatdoc! {r"
            INSERT IGNORE
            INTO `{TABLE_GROUP_MEMBERS}` (`group_id`, `user_id`)
            VALUES {values_arg}
        "};
        let query = gms.iter().fold(sqlx::query(&query), |q, gm| {
            q.bind(gm.group_id.to_string()).bind(gm.user_id.to_string())
        });
        query.execute(&self.0).await?;
        Ok(())
    }

    pub(crate) async fn update_group_member(
        &self,
        gid: &GroupId,
        uid: &UserId,
        gm: GroupMember,
    ) -> Result<()> {
        let query = formatdoc! {r"
            UPDATE `{TABLE_GROUP_MEMBERS}`
            SET `group_id` = ?, `user_id` = ?
            WHERE `group_id` = ?, `user_id` = ?
        "};
        sqlx::query(&query)
            .bind(gm.group_id.to_string())
            .bind(gm.user_id.to_string())
            .bind(gid.to_string())
            .bind(uid.to_string())
            .execute(&self.0)
            .await?;
        Ok(())
    }

    pub(crate) async fn delete_group_member(&self, gm: GroupMember) -> Result<()> {
        let query = formatdoc! {r"
            DELETE FROM `{TABLE_GROUP_MEMBERS}`
            WHERE `group_id` = ?, `user_id` = ?
        "};
        sqlx::query(&query)
            .bind(gm.group_id.to_string())
            .bind(gm.user_id.to_string())
            .execute(&self.0)
            .await?;
        Ok(())
    }
}
