use sqlx::Row;

mod group;
mod group_member;
mod owner;
mod user;
mod webhook;

pub use group::Group;
pub use group_member::GroupMember;
pub use owner::Owner;
pub use user::User;
pub use webhook::Webhook;

fn parse_col_str2uuid<Id>(row: &sqlx::mysql::MySqlRow, col: &str) -> sqlx::Result<Id>
where
    Id: From<uuid::Uuid>,
{
    let id_str = row.try_get(col)?;
    let id = uuid::Uuid::parse_str(id_str).map_err(|e| sqlx::Error::ColumnDecode {
        index: col.to_string(),
        source: e.into(),
    })?;
    Ok(id.into())
}
