use crate::{
    api::v1::{
        infos::ModiyfUserInfo,
        message::{FriendRequest, SearchResponse},
    },
    internal::model::{group::Group, user::User, user_friend::UserFriend},
};
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

pub async fn get_user_details(uuid: String, pool: &MySqlPool) -> Result<User> {
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

pub async fn get_user_or_group_by_name(name: &str, pool: &MySqlPool) -> Result<SearchResponse> {
    let mut user = User::default();
    match User::get_by_username(name, pool).await {
        Ok(u) => {
            user.uuid = u.uuid;
            user.username = u.username;
            user.nickname = u.nickname;
            user.avatar = u.avatar;
        }
        _ => {}
    }

    let mut group = Group::default();
    match Group::get_by_name(name, pool).await {
        Ok(g) => {
            group.uuid = g.uuid;
            group.name = g.name;
        }
        _ => {}
    }

    Ok(SearchResponse { user, group })
}

pub async fn modify_user_info(info: ModiyfUserInfo, pool: &MySqlPool) -> Result<()> {
    let ret = match User::get_by_username(&info.Username, pool).await {
        Err(e) => Err(anyhow::anyhow!(
            "user {} doesn't exists:{:?}",
            info.Username,
            e
        )),
        Ok(mut user) => {
            user.nickname = info.Nickname;
            user.email = info.Email;
            user.password = info.Password;
            user.update(pool).await?;
            Ok(())
        }
    };
    ret
}

pub async fn modify_user_avatar(avatar: &str, uuid: &str, pool: &MySqlPool) -> Result<()> {
    let ret = match User::get_by_uuid(uuid, pool).await {
        Err(e) => Err(anyhow::anyhow!("user {} doesn't exists:{:?}", uuid, e)),
        Ok(mut user) => {
            user.avatar = avatar.to_owned();
            user.update(pool).await?;
            Ok(())
        }
    };
    ret
}

pub async fn add_friend(request: FriendRequest, pool: &MySqlPool) -> Result<()> {
    let query_user = User::get_by_uuid(&request.uuid, pool).await?;
    let friend_user = User::get_by_username(&request.friendUsername, pool).await?;
    if let Ok(_) = UserFriend::get_by_user_id_friend_id(query_user.id, friend_user.id, pool).await {
        return Err(anyhow::anyhow!("already friend"));
    }
    let user_friend = UserFriend {
        user_id: query_user.id,
        friend_id: friend_user.id,
        ..Default::default()
    };
    user_friend.insert(pool).await?;
    Ok(())
}
