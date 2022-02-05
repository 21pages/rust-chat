use crate::internal::model::user::User;
use anyhow::Result;
use sqlx::{self, MySqlPool};
use tracing::trace;
use uuid::Uuid;

pub async fn login(user: &mut User, pool: &MySqlPool) -> Result<()> {
    *user = User::get_by_username_password(&user.username, &user.password, pool).await?;
    Ok(())
}

pub async fn register(user: &mut User, pool: &MySqlPool) -> Result<()> {
    if User::exist_by_username(&user.username, pool).await? {
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
    let user_all_info = User::get_by_uuid(&uuid, pool).await?;
    let mut user = User::default();
    user.uuid = user_all_info.uuid;
    user.username = user_all_info.username;
    user.nickname = user_all_info.nickname;
    user.avatar = user_all_info.avatar;
    Ok(user)
}

pub async fn get_user_list(uuid: String, pool: &MySqlPool) -> Result<Vec<User>> {
    let user = User::get_by_uuid(&uuid, pool).await?;
    let infos = user.get_friend_user_infos(pool).await?;
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
