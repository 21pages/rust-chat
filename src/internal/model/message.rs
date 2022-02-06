use crate::{
    api::v1::message::{GroupMessageResponse, UserMessageResponse},
    common::date_format::{my_date_format, option_date_format},
};
use anyhow::Result;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::{self, MySqlPool};

use super::user::User;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub id: i32,
    #[serde(with = "my_date_format")]
    pub created_at: DateTime<Local>,
    #[serde(with = "option_date_format")]
    pub updated_at: Option<DateTime<Local>>,
    pub deleted_at: u64,
    pub from_user_id: i32,
    pub to_user_id: i32,
    pub content: String,
    pub messsage_type: i16,
    pub content_type: i16,
    pub pic: String,
    pub url: String,
}

impl Message {
    pub async fn get_user_message(
        current: &User,
        friend: &User,
        pool: &MySqlPool,
    ) -> Result<Vec<UserMessageResponse>> {
        let msgs = sqlx::query_as(
            r#"
            SELECT m.id, m.from_user_id, m.to_user_id, m.content, m.content_type, m.url, m.created_at, u.username AS from_username, 
            u.avatar, to_user.username AS to_username  
            FROM messages AS m 
            LEFT JOIN users AS u ON m.from_user_id = u.id 
            LEFT JOIN users AS to_user ON m.to_user_id = to_user.id 
            WHERE from_user_id IN (?, ?) AND to_user_id IN (?, ?)"#
        )
        .bind(current.id)
        .bind(friend.id)
        .bind(current.id)
        .bind(friend.id)
        .fetch_all(pool)
        .await?;
        Ok(msgs)
    }

    pub async fn get_group_message(id: i32, pool: &MySqlPool) -> Result<Vec<GroupMessageResponse>> {
        let msgs = sqlx::query_as(
            r#"
            SELECT m.id, m.from_user_id, m.to_user_id, m.content, m.content_type, 
            m.url, m.created_at, u.username AS from_username, u.avatar 
            FROM messages AS m 
            LEFT JOIN users AS u 
            ON m.from_user_id = u.id 
            WHERE m.message_type = 2 AND m.to_user_id = ?"#,
        )
        .bind(id)
        .fetch_all(pool)
        .await?;
        Ok(msgs)
    }
}
