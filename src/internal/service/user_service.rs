use crate::internal::model::user::User;
use anyhow::Result;
use serde::Deserialize;
use sqlx::{self, MySqlPool};
use tracing::trace;

pub async fn login(user: &mut User, pool: &MySqlPool) -> Result<()> {
    *user = sqlx::query_as::<_, User>("select * from users where username=? and password=?")
        .bind(&user.username)
        .bind(&user.password)
        .fetch_one(pool)
        .await?;
    trace!("{:?}", user);
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
    trace!("{:?}", user);
    Ok(user)
}

#[derive(sqlx::Type, sqlx::FromRow)]
#[sqlx(transparent)]
struct MyInt32(i32);

#[derive(sqlx::Type, sqlx::FromRow)]
#[sqlx(transparent)]
struct MyVecUser(Vec<User>);

#[derive(Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
struct FriendUserInfo {
    pub uuid: String,
    pub username: String,
    pub avatar: String,
}

pub async fn get_user_list(uuid: String, pool: &MySqlPool) -> Result<Vec<User>> {
    let id: MyInt32 = sqlx::query_as::<_, MyInt32>("select id from users where uuid=?")
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
