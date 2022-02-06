use crate::common::{
    constant,
    date_format::{my_date_format, option_date_format},
};
use anyhow::Result;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::{self, MySqlPool};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct UserFriend {
    pub id: i32,
    #[serde(with = "my_date_format")]
    pub created_at: DateTime<Local>,
    #[serde(with = "option_date_format")]
    pub updated_at: Option<DateTime<Local>>,
    pub deleted_at: u64,
    pub user_id: i32,
    pub friend_id: i32,
}

impl Default for UserFriend {
    fn default() -> Self {
        Self {
            id: *constant::INVALID_ID,
            created_at: chrono::offset::Local::now(),
            updated_at: Default::default(),
            deleted_at: Default::default(),
            user_id: Default::default(),
            friend_id: Default::default(),
        }
    }
}

impl UserFriend {
    pub async fn get_by_user_id_friend_id(
        user_id: i32,
        friend_id: i32,
        pool: &MySqlPool,
    ) -> Result<Self> {
        let f: Self =
            sqlx::query_as("select * from `user_friends` where user_id=? and friend_id=?")
                .bind(user_id)
                .bind(friend_id)
                .fetch_one(pool)
                .await?;
        Ok(f)
    }

    pub async fn insert(&self, pool: &MySqlPool) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO
            `user_friends` (created_at, deleted_at, updated_at, user_id, friend_id)
            VALUES ( ?,?,?,?,? )
            "#,
            self.created_at,
            self.deleted_at,
            self.updated_at,
            self.user_id,
            self.friend_id
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}
