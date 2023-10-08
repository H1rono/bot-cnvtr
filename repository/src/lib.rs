use sqlx::migrate::Migrator;
use sqlx::mysql::MySqlRow;
use sqlx::Row;
use uuid::Uuid;

mod config;
pub mod model;
mod repo_impl;

pub use config::Config;

pub use model::group::Group;
pub use model::group_member::GroupMember;
pub use model::owner::Owner;
pub use model::user::User;
pub use model::webhook::Webhook;

pub use repo_impl::RepositoryImpl;

pub const MIGRATOR: Migrator = sqlx::migrate!("../migrations");

fn parse_col_str2uuid(row: &MySqlRow, col: &str) -> sqlx::Result<Uuid> {
    row.try_get(col).and_then(|u| {
        Uuid::parse_str(u).map_err(|e| sqlx::Error::ColumnDecode {
            index: col.to_string(),
            source: e.into(),
        })
    })
}
