use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use time::OffsetDateTime;

// https://docs.rs/sqlx/latest/sqlx/trait.FromRow.html#default
#[derive(Default, Debug, Serialize, Deserialize, FromRow)]
#[sqlx(default)]
pub struct Setting {
    pub id: u64,
    pub user_id: u64,
    pub key: String,
    pub value: Value,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct SettingValuePush {
    pub push_token: String,
}
