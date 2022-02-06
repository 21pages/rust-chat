use crate::{
    api::v1::{infos::GroupUserInfo, message::GroupResponse},
    common::{
        constant,
        date_format::{self, my_date_format, option_date_format},
    },
    internal::db::sqlx_adapter,
};
use anyhow::Result;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::{self, MySqlPool};

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

impl Default for Group {
    fn default() -> Self {
        Self {
            id: *constant::INVALID_ID,
            uuid: Default::default(),
            created_at: *date_format::INVALID_DATE,
            updated_at: None,
            deleted_at: 0,
            user_id: Default::default(),
            name: Default::default(),
            notice: Default::default(),
        }
    }
}

impl Group {
    pub async fn get_user_groups(id: i32, pool: &MySqlPool) -> Result<Vec<GroupResponse>> {
        let groups: Vec<GroupResponse> = sqlx::query_as(
            r#"
            SELECT g.id AS group_id, g.uuid, g.created_at, g.name, g.notice 
            FROM group_members AS gm 
            LEFT JOIN `groups` AS g 
            ON gm.group_id = g.id 
            WHERE gm.user_id = ?
            "#,
        )
        .bind(id)
        .fetch_all(pool)
        .await?;
        Ok(groups)
    }

    pub async fn get_by_name(name: &str, pool: &MySqlPool) -> Result<Self> {
        let group: Group = sqlx::query_as("select * from `groups` where name=?")
            .bind(name)
            .fetch_one(pool)
            .await?;
        Ok(group)
    }

    pub async fn get_by_uuid(uuid: &str, pool: &MySqlPool) -> Result<Self> {
        let group: Group = sqlx::query_as("select * from `groups` where uuid=?")
            .bind(uuid)
            .fetch_one(pool)
            .await?;
        Ok(group)
    }

    pub async fn exist_by_name(name: &str, pool: &MySqlPool) -> Result<bool> {
        let count: sqlx_adapter::MyInt32 =
            sqlx::query_as("select count(*) from `groups` where name=?")
                .bind(name)
                .fetch_one(pool)
                .await?;
        Ok(count.0 > 0)
    }

    pub async fn insert(&self, pool: &MySqlPool) -> Result<()> {
        //group is keyword
        sqlx::query!(
            r#"
            INSERT INTO
            `groups` (uuid, created_at, deleted_at, updated_at, user_id, name, notice)
            VALUES ( ?,?,?,?,?,?,? )
            "#,
            self.uuid,
            self.created_at,
            self.deleted_at,
            self.updated_at,
            self.user_id,
            self.name,
            self.notice
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn get_users(id: i32, pool: &MySqlPool) -> Result<Vec<GroupUserInfo>> {
        let infos: Vec<GroupUserInfo> = sqlx::query_as(
            r#"
        SELECT u.uuid, u.avatar, u.username 
        FROM `groups` AS g 
        JOIN group_members AS gm 
        ON gm.group_id = g.id 
        JOIN users AS u 
        ON u.id = gm.user_id 
        WHERE g.id = ?"#,
        )
        .bind(id)
        .fetch_all(pool)
        .await?;
        Ok(infos)
    }
}
