use indoc::indoc;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Result};
use uuid::Uuid;

use super::Database;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, FromRow)]
pub struct GroupMember {
    pub group_id: Uuid,
    pub user_id: Uuid,
}

impl Database {
    pub async fn read_group_members(&self) -> Result<Vec<GroupMember>> {
        sqlx::query(indoc! {r#"
            SELECT *
            FROM `group_members`
        "#})
        .fetch_all(&self.0)
        .await?
        .iter()
        .map(GroupMember::from_row)
        .collect::<Result<_>>()
    }

    pub async fn find_group_member(&self, gid: &Uuid, uid: &Uuid) -> Result<Option<GroupMember>> {
        sqlx::query(indoc! {r#"
            SELECT *
            FROM `group_members`
            WHERE `group_id` = ?, `user_id` = ?
            LIMIT 1
        "#})
        .bind(gid)
        .bind(uid)
        .fetch_optional(&self.0)
        .await?
        .map(|gm| GroupMember::from_row(&gm))
        .transpose()
    }

    pub async fn create_group_member(&self, gm: GroupMember) -> Result<()> {
        sqlx::query(indoc! {r#"
            INSERT
            INTO `group_members` (`group_id`, `user_id`)
            VALUES (?, ?)
        "#})
        .bind(gm.group_id)
        .bind(gm.user_id)
        .execute(&self.0)
        .await?;
        Ok(())
    }

    pub async fn update_group_member(&self, gid: &Uuid, uid: &Uuid, gm: GroupMember) -> Result<()> {
        sqlx::query(indoc! {r#"
            UPDATE `group_members`
            SET `group_id` = ?, `user_id` = ?
            WHERE `group_id` = ?, `user_id` = ?
        "#})
        .bind(gm.group_id)
        .bind(gm.user_id)
        .bind(gid)
        .bind(uid)
        .execute(&self.0)
        .await?;
        Ok(())
    }

    pub async fn delete_group_membed(&self, gm: GroupMember) -> Result<()> {
        sqlx::query(indoc! {r#"
            DELETE FROM `group_members`
            WHERE `group_id` = ?, `user_id` = ?
        "#})
        .bind(gm.group_id)
        .bind(gm.user_id)
        .execute(&self.0)
        .await?;
        Ok(())
    }
}
