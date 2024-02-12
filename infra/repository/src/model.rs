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

fn parse_col_str2uuid(row: &sqlx::mysql::MySqlRow, col: &str) -> sqlx::Result<uuid::Uuid> {
    row.try_get(col).and_then(|u| {
        uuid::Uuid::parse_str(u).map_err(|e| sqlx::Error::ColumnDecode {
            index: col.to_string(),
            source: e.into(),
        })
    })
}
