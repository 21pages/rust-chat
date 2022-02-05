use crate::internal::{db::sqlx_adapter, model::user::User};
use anyhow::Result;
use serde::Deserialize;
use sqlx::{self, MySqlPool};
use tracing::trace;
use uuid::Uuid;

pub async fn login(user: &mut User, pool: &MySqlPool) -> Result<()> {
    *user = sqlx::query_as::<_, User>("select * from users where username=? and password=?")
        .bind(&user.username)
        .bind(&user.password)
        .fetch_one(pool)
        .await?;
    Ok(())
}

/*
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
*/

pub async fn register(user: &mut User, pool: &MySqlPool) -> Result<()> {
    let count: sqlx_adapter::MyInt32 =
        sqlx::query_as("select count(*) from users where username=?")
            .bind(&user.username)
            .fetch_one(pool)
            .await?;
    trace!("{} count:{}", user.username, count.0);
    if count.0 > 0 {
        return Err(anyhow::anyhow!("user already exists"));
    }
    user.uuid = Uuid::new_v4().to_string();
    user.create_at = chrono::offset::Local::now();
    user.delete_at = 0;
    user.insert(pool).await?;
    Ok(())
}

pub async fn get_user_details(
    uuid: String,
    pool: &MySqlPool,
) -> Result<User, Box<dyn std::error::Error>> {
    let user_all_info = sqlx::query_as::<_, User>("select * from users where uuid=?")
        .bind(uuid)
        .fetch_one(pool)
        .await?;
    let mut user = User::default();
    user.uuid = user_all_info.uuid;
    user.username = user_all_info.username;
    user.nickname = user_all_info.nickname;
    user.avatar = user_all_info.avatar;
    Ok(user)
}

#[derive(Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
struct FriendUserInfo {
    pub uuid: String,
    pub username: String,
    pub avatar: String,
}

pub async fn get_user_list(uuid: String, pool: &MySqlPool) -> Result<Vec<User>> {
    let id: sqlx_adapter::MyInt32 = sqlx::query_as("select id from users where uuid=?")
        .bind(uuid)
        .fetch_one(pool)
        .await?;
    let infos :Vec<FriendUserInfo> = sqlx::query_as("SELECT u.username, u.uuid, u.avatar FROM user_friends AS uf JOIN users AS u ON uf.friend_id = u.id WHERE uf.user_id = ?")
    .bind(id.0)
    .fetch_all(pool)
    .await?;
    let mut users = vec![];
    for info in infos.into_iter() {
        let mut user = User::default();
        user.username = info.username;
        user.uuid = info.uuid;
        user.avatar = info.avatar;
        users.push(user);
    }
    trace!("user list: {:?}", users);
    Ok(users)
}
