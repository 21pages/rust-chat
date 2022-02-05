use crate::common::date_format::{my_date_format, option_date_format};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    pub id: i32,
    pub uuid: String,
    #[serde(with = "my_date_format")]
    pub created_at: DateTime<Local>,
    #[serde(with = "option_date_format")]
    pub updated_at: Option<DateTime<Local>>,
    pub deleted_at: u64,
    pub user_id: i32,
    pub name: String,
    pub notice: String,
}
