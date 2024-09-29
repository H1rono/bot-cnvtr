use std::fmt::Display;
use std::iter;
use std::str::FromStr;

use indoc::formatdoc;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlRow, FromRow};
use uuid::Uuid;

use domain::{OwnerId, OwnerKind};

use crate::error::{Error, Result};
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
    pub(crate) async fn read_owners(&self) -> Result<Vec<Owner>> {
        let query = formatdoc! {r"
            SELECT *
            FROM `{TABLE_OWNERS}`
        "};
        sqlx::query_as(&query)
            .fetch_all(&self.0)
            .await
            .map_err(Error::from)
    }

    pub(crate) async fn find_owner(&self, id: &OwnerId) -> Result<Option<Owner>> {
        let query = formatdoc! {r"
            SELECT *
            FROM `{TABLE_OWNERS}`
            WHERE `id` = ?
            LIMIT 1
        "};
        sqlx::query_as(&query)
            .bind(id.to_string())
            .fetch_optional(&self.0)
            .await
            .map_err(Error::from)
    }

    pub(crate) async fn create_owner(&self, o: Owner) -> Result<()> {
        let query = formatdoc! {r"
            INSERT INTO `{TABLE_OWNERS}` (`id`, `name`, `group`)
            VALUES (?, ?, ?)
        "};
        sqlx::query(&query)
            .bind(o.id.to_string())
            .bind(o.name)
            .bind(OwnerKindCol::from(o.kind))
            .execute(&self.0)
            .await?;
        Ok(())
    }

    pub(crate) async fn create_ignore_owners(&self, os: &[Owner]) -> Result<()> {
        if os.is_empty() {
            return Ok(());
        }
        let values_arg = iter::repeat("(?, ?, ?)").take(os.len()).join(", ");
        let query = formatdoc! {r"
            INSERT IGNORE
            INTO `{TABLE_OWNERS}` (`id`, `name`, `group`)
            VALUES {values_arg}
        "};
        let query = os.iter().fold(sqlx::query(&query), |q, o| {
            q.bind(o.id.to_string())
                .bind(&o.name)
                .bind(OwnerKindCol::from(o.kind))
        });
        query.execute(&self.0).await?;
        Ok(())
    }

    pub(crate) async fn update_owner(&self, id: &OwnerId, o: Owner) -> Result<()> {
        let query = formatdoc! {r"
            UPDATE `{TABLE_OWNERS}`
            SET `id` = ?, `name` = ?, `group` = ?
            WHERE `id` = ?
        "};
        sqlx::query(&query)
            .bind(o.id.to_string())
            .bind(o.name)
            .bind(OwnerKindCol::from(o.kind))
            .bind(id.to_string())
            .execute(&self.0)
            .await?;
        Ok(())
    }

    pub(crate) async fn delete_owner(&self, id: &OwnerId) -> Result<()> {
        let query = formatdoc! {r"
            DELETE FROM `{TABLE_OWNERS}`
            WHERE `id` = ?
        "};
        sqlx::query(&query)
            .bind(id.to_string())
            .execute(&self.0)
            .await?;
        Ok(())
    }
}
