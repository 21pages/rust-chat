use crate::{
    common::date_format::{my_date_format, option_date_format},
    internal::db::sqlx_adapter,
};
use anyhow::Result;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::{self, MySqlPool};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct GroupMember {
    pub id: i32,
    #[serde(with = "my_date_format")]
    pub created_at: DateTime<Local>,
    #[serde(with = "option_date_format")]
    pub updated_at: Option<DateTime<Local>>,
    pub deleted_at: u64,
    pub user_id: i32,
    pub group_id: i32,
    pub nickname: String,
    pub mute: i16,
}
impl Default for GroupMember {
    fn default() -> Self {
        Self {
            id: Default::default(),
            created_at: chrono::offset::Local::now(),
            updated_at: Default::default(),
            deleted_at: Default::default(),
            user_id: Default::default(),
            group_id: Default::default(),
            nickname: Default::default(),
            mute: Default::default(),
        }
    }
}

impl GroupMember {
    pub async fn exist_by_user_id_group_id(
        user_id: i32,
        group_id: i32,
        pool: &MySqlPool,
    ) -> Result<bool> {
        let count: sqlx_adapter::MyInt32 =
            sqlx::query_as("select count(*) from group_members where user_id=? and group_id=?")
                .bind(user_id)
                .bind(group_id)
                .fetch_one(pool)
                .await?;
        Ok(count.0 > 0)
    }

    pub async fn insert(&self, pool: &MySqlPool) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO
            group_members (created_at, updated_at, deleted_at, user_id, group_id, nickname, mute)
            VALUES ( ?,?,?,?,?,?,? )
            "#,
            self.created_at,
            self.updated_at,
            self.deleted_at,
            self.user_id,
            self.group_id,
            self.nickname,
            self.mute,
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}
