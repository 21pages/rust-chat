use crate::common::date_format::{my_date_format, option_date_format};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct GroupMember {
    id: i32,
    #[serde(with = "my_date_format")]
    create_at: DateTime<Local>,
    #[serde(with = "option_date_format")]
    update_at: Option<DateTime<Local>>,
    delete_at: u64,
    user_id: i32,
    group_id: i32,
    nickname: String,
    mute: i16,
}
