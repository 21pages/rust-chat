use crate::{
    api::v1::infos,
    common::date_format::{self, my_date_format, option_date_format},
    internal::db::sqlx_adapter,
};
use anyhow::Result;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{self, MySqlPool};
use tracing::trace;

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

    pub async fn get_by_username(username: &str, pool: &MySqlPool) -> Result<User> {
        let user: User = sqlx::query_as("select * from users where username=?")
            .bind(username)
            .fetch_one(pool)
            .await?;
        Ok(user)
    }

    pub async fn get_by_username_password(
        username: &str,
        password: &str,
        pool: &MySqlPool,
    ) -> Result<User> {
        let user: User = sqlx::query_as("select * from users where username=? and password=?")
            .bind(username)
            .bind(password)
            .fetch_one(pool)
            .await?;
        Ok(user)
    }

    pub async fn get_by_uuid(uuid: &str, pool: &MySqlPool) -> Result<User> {
        let user: User = sqlx::query_as("select * from users where uuid=?")
            .bind(uuid)
            .fetch_one(pool)
            .await?;
        Ok(user)
    }

    pub async fn exist_by_username(username: &str, pool: &MySqlPool) -> Result<bool> {
        let count: sqlx_adapter::MyInt32 =
            sqlx::query_as("select count(*) from users where username=?")
                .bind(username)
                .fetch_one(pool)
                .await?;
        trace!("{} count:{}", username, count.0);
        Ok(count.0 > 0)
    }

    #[allow(unused)]
    pub async fn exist_by_uuid(uuid: &str, pool: &MySqlPool) -> Result<bool> {
        let count: sqlx_adapter::MyInt32 =
            sqlx::query_as("select count(*) from users where uuid=?")
                .bind(uuid)
                .fetch_one(pool)
                .await?;
        trace!("{} count:{}", uuid, count.0);
        Ok(count.0 > 0)
    }

    pub async fn get_friend_user_infos(
        &self,
        pool: &MySqlPool,
    ) -> Result<Vec<infos::FriendUserInfo>> {
        let infos: Vec<infos::FriendUserInfo> = sqlx::query_as(
            r#"
            SELECT u.username, u.uuid, u.avatar 
            FROM user_friends AS uf 
            JOIN users AS u ON uf.friend_id = u.id 
            WHERE uf.user_id = ?
            "#,
        )
        .bind(&self.id)
        .fetch_all(pool)
        .await?;
        Ok(infos)
    }
}
