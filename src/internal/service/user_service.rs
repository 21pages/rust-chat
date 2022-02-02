use crate::internal::model::user::User;
use sqlx::{self, MySqlPool};
use std::sync::Arc;
use tracing::trace;

pub async fn login(
    user: &mut User,
    pool: Arc<MySqlPool>,
) -> Result<(), Box<dyn std::error::Error>> {
    *user = sqlx::query_as::<_, User>("select * from users where username=? and password=?")
        .bind(&user.username)
        .bind(&user.password)
        .fetch_one(&*pool)
        .await?;
    trace!("{:?}", user);
    Ok(())
}

// #[derive(Debug, Serialize, sqlx::FromRow)]
// #[serde(rename_all = "camelCase")]
// pub struct UserDetails {
//     pub uuid: String,
//     pub username: String,
//     pub nickname: String,
//     pub avatar: String,
// }

// pub async fn get_user_details(
//     uuid: String,
//     pool: Arc<MySqlPool>,
// ) -> Result<User, Box<dyn std::error::Error>> {
//     let details = sqlx::query_as::<_, UserDetails>(
//         "select uuid, username, nickname, avatar from users where uuid=?",
//     )
//     .bind(uuid)
//     .fetch_one(&*pool)
//     .await?;
//     let mut user = User::default();
//     user.uuid = details.uuid;
//     user.username = details.username;
//     user.nickname = details.nickname;
//     user.avatar = details.avatar;
//     trace!("{:?}", user);
//     Ok(user)
// }

pub async fn get_user_details(
    uuid: String,
    pool: Arc<MySqlPool>,
) -> Result<User, Box<dyn std::error::Error>> {
    let user_all_info = sqlx::query_as::<_, User>("select * from users where uuid=?")
        .bind(uuid)
        .fetch_one(&*pool)
        .await?;
    let mut user = User::default();
    user.uuid = user_all_info.uuid;
    user.username = user_all_info.username;
    user.nickname = user_all_info.nickname;
    user.avatar = user_all_info.avatar;
    trace!("{:?}", user);
    Ok(user)
}
