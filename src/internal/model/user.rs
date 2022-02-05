use crate::common::date_format::{self, my_date_format, option_date_format};
use anyhow::Result;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{self, MySqlPool};

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

    pub async fn insert(&self, pool: &MySqlPool) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO
            users (uuid, username, password, nickname, avatar, email, create_at, delete_at)
            VALUES ( ?,?,?,?,?,?,?,? )
            "#,
            self.uuid,
            self.username,
            self.password,
            self.nickname,
            self.avatar,
            self.email,
            self.create_at,
            self.delete_at
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}
