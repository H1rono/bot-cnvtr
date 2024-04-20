use std::iter;

use indoc::{formatdoc, indoc};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlRow, FromRow, Result};
use uuid::Uuid;

use super::parse_col_str2uuid;
use crate::RepositoryImpl;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct GroupMember {
    pub group_id: Uuid,
    pub user_id: Uuid,
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
        sqlx::query_as(indoc! {r#"
            SELECT *
            FROM `group_members`
        "#})
        .fetch_all(&self.0)
        .await
    }

    pub(crate) async fn find_group_member(
        &self,
        gid: &Uuid,
        uid: &Uuid,
    ) -> Result<Option<GroupMember>> {
        sqlx::query_as(indoc! {r#"
            SELECT *
            FROM `group_members`
            WHERE `group_id` = ?, `user_id` = ?
            LIMIT 1
        "#})
        .bind(gid.to_string())
        .bind(uid.to_string())
        .fetch_optional(&self.0)
        .await
    }

    pub(crate) async fn filter_group_members_by_gid(&self, gid: &Uuid) -> Result<Vec<GroupMember>> {
        sqlx::query_as(indoc! {r#"
            SELECT *
            FROM `group_members`
            WHERE `group_id` = ?
        "#})
        .bind(gid.to_string())
        .fetch_all(&self.0)
        .await
    }

    pub(crate) async fn filter_group_members_by_uid(&self, uid: &Uuid) -> Result<Vec<GroupMember>> {
        sqlx::query_as(indoc! {r#"
            SELECT *
            FROM `group_members`
            WHERE `user_id` = ?
        "#})
        .bind(uid.to_string())
        .fetch_all(&self.0)
        .await
    }

    pub(crate) async fn create_group_member(&self, gm: GroupMember) -> Result<()> {
        sqlx::query(indoc! {r#"
            INSERT
            INTO `group_members` (`group_id`, `user_id`)
            VALUES (?, ?)
        "#})
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
        let query = formatdoc! {
            r#"
                INSERT IGNORE
                INTO `group_members` (`group_id`, `user_id`)
                VALUES {}
            "#,
            iter::repeat("(?, ?)").take(gms.len()).join(", ")
        };
        let query = gms.iter().fold(sqlx::query(&query), |q, gm| {
            q.bind(gm.group_id.to_string()).bind(gm.user_id.to_string())
        });
        query.execute(&self.0).await?;
        Ok(())
    }

    pub(crate) async fn update_group_member(
        &self,
        gid: &Uuid,
        uid: &Uuid,
        gm: GroupMember,
    ) -> Result<()> {
        sqlx::query(indoc! {r#"
            UPDATE `group_members`
            SET `group_id` = ?, `user_id` = ?
            WHERE `group_id` = ?, `user_id` = ?
        "#})
        .bind(gm.group_id.to_string())
        .bind(gm.user_id.to_string())
        .bind(gid.to_string())
        .bind(uid.to_string())
        .execute(&self.0)
        .await?;
        Ok(())
    }

    pub(crate) async fn delete_group_member(&self, gm: GroupMember) -> Result<()> {
        sqlx::query(indoc! {r#"
            DELETE FROM `group_members`
            WHERE `group_id` = ?, `user_id` = ?
        "#})
        .bind(gm.group_id.to_string())
        .bind(gm.user_id.to_string())
        .execute(&self.0)
        .await?;
        Ok(())
    }
}
