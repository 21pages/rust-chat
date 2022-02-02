use crate::common::date_format::{my_date_format, option_date_format};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub id: i32,
    #[serde(with = "my_date_format")]
    pub create_at: DateTime<Local>,
    #[serde(with = "option_date_format")]
    pub update_at: Option<DateTime<Local>>,
    pub delete_at: u64,
    pub from_user_id: i32,
    pub to_user_id: i32,
    pub content: String,
    pub messsage_type: i16,
    pub content_type: i16,
    pub pic: String,
    pub url: String,
}
