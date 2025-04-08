use anyhow::Context;
use indoc::formatdoc;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, mysql::MySqlRow};
use uuid::Uuid;

use domain::{Failure, GroupId, UserId};

use crate::RepositoryImpl;

const TABLE_GROUP_MEMBERS: &str = "group_members_v2";

// FIXME: マクロ使いたいここ
#[must_use]
#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
struct GroupMemberRow {
    pub group_id: Uuid,
    pub user_id: Uuid,
}

#[must_use]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct GroupMember {
    pub group_id: GroupId,
    pub user_id: UserId,
}

impl From<GroupMemberRow> for GroupMember {
    fn from(value: GroupMemberRow) -> Self {
        let GroupMemberRow { group_id, user_id } = value;
        Self {
            group_id: group_id.into(),
            user_id: user_id.into(),
        }
    }
}

impl From<GroupMember> for GroupMemberRow {
    fn from(value: GroupMember) -> Self {
        let GroupMember { group_id, user_id } = value;
        Self {
            group_id: group_id.into(),
            user_id: user_id.into(),
        }
    }
}

impl<'r> FromRow<'r, MySqlRow> for GroupMember {
    fn from_row(row: &'r MySqlRow) -> std::result::Result<Self, sqlx::Error> {
        GroupMemberRow::from_row(row).map(Self::from)
    }
}

#[allow(dead_code)]
impl RepositoryImpl {
    pub(crate) async fn read_group_members(&self) -> Result<Vec<GroupMember>, Failure> {
        let query = formatdoc! {r"
            SELECT *
            FROM `{TABLE_GROUP_MEMBERS}`
        "};
        let res = sqlx::query_as(&query)
            .fetch_all(&self.0)
            .await
            .context("Failed to read group members from DB")?;
        Ok(res)
    }

    pub(crate) async fn find_group_member(
        &self,
        gid: &GroupId,
        uid: &UserId,
    ) -> Result<GroupMember, Failure> {
        let query = formatdoc! {r"
            SELECT *
            FROM `{TABLE_GROUP_MEMBERS}`
            WHERE `group_id` = ?, `user_id` = ?
            LIMIT 1
        "};
        sqlx::query_as(&query)
            .bind(gid.0)
            .bind(uid.0)
            .fetch_optional(&self.0)
            .await
            .context("Failed to read a group member from DB")?
            .ok_or_else(|| Failure::reject_not_found("No group member found"))
    }

    pub(crate) async fn filter_group_members_by_gid(
        &self,
        gid: &GroupId,
    ) -> Result<Vec<GroupMember>, Failure> {
        let query = formatdoc! {r"
            SELECT *
            FROM `{TABLE_GROUP_MEMBERS}`
            WHERE `group_id` = ?
        "};
        let res = sqlx::query_as(&query)
            .bind(gid.0)
            .fetch_all(&self.0)
            .await
            .context("Failed to read-filter group members from DB")?;
        Ok(res)
    }

    pub(crate) async fn filter_group_members_by_uid(
        &self,
        uid: &UserId,
    ) -> Result<Vec<GroupMember>, Failure> {
        let query = formatdoc! {r"
            SELECT *
            FROM `{TABLE_GROUP_MEMBERS}`
            WHERE `user_id` = ?
        "};
        let res = sqlx::query_as(&query)
            .bind(uid.0)
            .fetch_all(&self.0)
            .await
            .context("Failed to read-filter group members from DB")?;
        Ok(res)
    }

    pub(crate) async fn create_group_member(&self, gm: GroupMember) -> Result<(), Failure> {
        let query = formatdoc! {r"
            INSERT
            INTO `{TABLE_GROUP_MEMBERS}` (`group_id`, `user_id`)
            VALUES (?, ?)
        "};
        sqlx::query(&query)
            .bind(gm.group_id.0)
            .bind(gm.user_id.0)
            .execute(&self.0)
            .await
            .context("Failed to create group member to DB")?;
        Ok(())
    }

    pub(crate) async fn create_ignore_group_members(
        &self,
        gms: &[GroupMember],
    ) -> Result<(), Failure> {
        if gms.is_empty() {
            return Ok(());
        }
        let values_arg = std::iter::repeat_n("(?, ?)", gms.len()).join(", ");
        let query = formatdoc! {r"
            INSERT IGNORE
            INTO `{TABLE_GROUP_MEMBERS}` (`group_id`, `user_id`)
            VALUES {values_arg}
        "};
        let query = gms.iter().fold(sqlx::query(&query), |q, gm| {
            q.bind(gm.group_id.0).bind(gm.user_id.0)
        });
        query
            .execute(&self.0)
            .await
            .context("Failed to create group members to DB")?;
        Ok(())
    }

    pub(crate) async fn update_group_member(
        &self,
        gid: &GroupId,
        uid: &UserId,
        gm: GroupMember,
    ) -> Result<(), Failure> {
        let query = formatdoc! {r"
            UPDATE `{TABLE_GROUP_MEMBERS}`
            SET `group_id` = ?, `user_id` = ?
            WHERE `group_id` = ?, `user_id` = ?
        "};
        sqlx::query(&query)
            .bind(gm.group_id.0)
            .bind(gm.user_id.0)
            .bind(gid.0)
            .bind(uid.0)
            .execute(&self.0)
            .await
            .context("Failed to update group member in DB")?;
        Ok(())
    }

    pub(crate) async fn delete_group_member(&self, gm: GroupMember) -> Result<(), Failure> {
        let query = formatdoc! {r"
            DELETE FROM `{TABLE_GROUP_MEMBERS}`
            WHERE `group_id` = ?, `user_id` = ?
        "};
        sqlx::query(&query)
            .bind(gm.group_id.0)
            .bind(gm.user_id.0)
            .execute(&self.0)
            .await
            .context("Failed to delete group member from DB")?;
        Ok(())
    }
}
