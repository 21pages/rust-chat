use crate::common::date_format::{self, my_date_format, option_date_format};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i32,
    pub uuid: String,
    pub username: String,
    pub password: String,
    pub nickname: String,
    pub avatar: String,
    pub email: String,
    #[serde(with = "my_date_format")]
    pub create_at: DateTime<Local>,
    #[serde(with = "option_date_format")]
    pub update_at: Option<DateTime<Local>>,
    pub delete_at: i64,
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: Default::default(),
            uuid: Default::default(),
            username: Default::default(),
            password: Default::default(),
            nickname: Default::default(),
            avatar: Default::default(),
            email: Default::default(),
            create_at: *date_format::INVALID_DATE,
            update_at: None,
            delete_at: 0,
        }
    }
}

impl User {
    pub fn to_json_value(&self) -> Value {
        serde_json::to_value(self).unwrap()
    }
}
