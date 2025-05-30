use std::fmt::Display;
use std::str::FromStr;

use anyhow::Context;
use indoc::formatdoc;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, mysql::MySqlRow};
use uuid::Uuid;

use domain::{Failure, OwnerId, OwnerKind};

use crate::RepositoryImpl;

const TABLE_OWNERS: &str = "owners_v2";

#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, sqlx::Decode, sqlx::Encode)]
#[sqlx(rename_all = "snake_case")]
enum OwnerKindCol {
    Group,
    SingleUser,
}

#[must_use]
#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
struct OwnerRow {
    pub id: Uuid,
    pub name: String,
    pub kind: OwnerKindCol,
}

#[must_use]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Owner {
    pub id: OwnerId,
    pub name: String,
    pub kind: OwnerKind,
}

impl From<OwnerKindCol> for OwnerKind {
    fn from(value: OwnerKindCol) -> Self {
        match value {
            OwnerKindCol::Group => Self::Group,
            OwnerKindCol::SingleUser => Self::SingleUser,
        }
    }
}

impl From<OwnerKind> for OwnerKindCol {
    fn from(value: OwnerKind) -> Self {
        match value {
            OwnerKind::Group => Self::Group,
            OwnerKind::SingleUser => Self::SingleUser,
        }
    }
}

impl FromStr for OwnerKindCol {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "group" => Ok(Self::Group),
            "single_user" => Ok(Self::SingleUser),
            _ => Err("unexpected str value for OwnerKindCol".to_string()),
        }
    }
}

impl Display for OwnerKindCol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Group => "group",
            Self::SingleUser => "single_user",
        })
    }
}

impl sqlx::Type<sqlx::MySql> for OwnerKindCol {
    fn type_info() -> <sqlx::MySql as sqlx::Database>::TypeInfo {
        str::type_info()
    }

    fn compatible(ty: &<sqlx::MySql as sqlx::Database>::TypeInfo) -> bool {
        str::compatible(ty)
    }
}

impl From<OwnerRow> for Owner {
    fn from(value: OwnerRow) -> Self {
        let OwnerRow { id, name, kind } = value;
        #[allow(clippy::useless_conversion)]
        Self {
            id: id.into(),
            name: name.into(),
            kind: kind.into(),
        }
    }
}

impl<'r> FromRow<'r, MySqlRow> for Owner {
    fn from_row(row: &'r MySqlRow) -> std::result::Result<Self, sqlx::Error> {
        OwnerRow::from_row(row).map(Self::from)
    }
}

#[allow(dead_code)]
impl RepositoryImpl {
    pub(crate) async fn read_owners(&self) -> Result<Vec<Owner>, Failure> {
        let query = formatdoc! {r"
            SELECT *
            FROM `{TABLE_OWNERS}`
        "};
        let res = sqlx::query_as(&query)
            .fetch_all(&self.0)
            .await
            .context("Failed to read owners from DB")?;
        Ok(res)
    }

    pub(crate) async fn find_owner(&self, id: &OwnerId) -> Result<Owner, Failure> {
        let query = formatdoc! {r"
            SELECT *
            FROM `{TABLE_OWNERS}`
            WHERE `id` = ?
            LIMIT 1
        "};
        sqlx::query_as(&query)
            .bind(id.0)
            .fetch_optional(&self.0)
            .await
            .context("Failed to read an owner from DB")?
            .ok_or_else(|| Failure::reject_not_found("No owner found"))
    }

    pub(crate) async fn create_owner(&self, o: Owner) -> Result<(), Failure> {
        let query = formatdoc! {r"
            INSERT INTO `{TABLE_OWNERS}` (`id`, `name`, `kind`)
            VALUES (?, ?, ?)
        "};
        sqlx::query(&query)
            .bind(o.id.0)
            .bind(o.name)
            .bind(OwnerKindCol::from(o.kind))
            .execute(&self.0)
            .await
            .context("Failed to crate an owner to DB")?;
        Ok(())
    }

    pub(crate) async fn create_ignore_owners(&self, os: &[Owner]) -> Result<(), Failure> {
        if os.is_empty() {
            return Ok(());
        }
        let values_arg = std::iter::repeat_n("(?, ?, ?)", os.len()).join(", ");
        let query = formatdoc! {r"
            INSERT IGNORE
            INTO `{TABLE_OWNERS}` (`id`, `name`, `kind`)
            VALUES {values_arg}
        "};
        let query = os.iter().fold(sqlx::query(&query), |q, o| {
            q.bind(o.id.0)
                .bind(&o.name)
                .bind(OwnerKindCol::from(o.kind))
        });
        query
            .execute(&self.0)
            .await
            .context("Failed to create owners to DB")?;
        Ok(())
    }

    pub(crate) async fn update_owner(&self, id: &OwnerId, o: Owner) -> Result<(), Failure> {
        let query = formatdoc! {r"
            UPDATE `{TABLE_OWNERS}`
            SET `id` = ?, `name` = ?, `kind` = ?
            WHERE `id` = ?
        "};
        sqlx::query(&query)
            .bind(o.id.0)
            .bind(o.name)
            .bind(OwnerKindCol::from(o.kind))
            .bind(id.0)
            .execute(&self.0)
            .await
            .context("Failed to update an owner in DB")?;
        Ok(())
    }

    pub(crate) async fn delete_owner(&self, id: &OwnerId) -> Result<(), Failure> {
        let query = formatdoc! {r"
            DELETE FROM `{TABLE_OWNERS}`
            WHERE `id` = ?
        "};
        sqlx::query(&query)
            .bind(id.0)
            .execute(&self.0)
            .await
            .context("Failed to delete an owner from DB")?;
        Ok(())
    }
}
