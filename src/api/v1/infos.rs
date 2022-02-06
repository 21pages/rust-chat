use serde::{Deserialize, Serialize};
use sqlx;

#[derive(Deserialize, sqlx::FromRow)]
pub struct FriendUserInfo {
    pub uuid: String,
    pub username: String,
    pub avatar: String,
}

#[derive(Deserialize, Debug)]
pub struct GroupSave {
    pub name: String,
}

#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct GroupUserInfo {
    pub uuid: String,
    pub username: String,
    pub avatar: String,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct ModiyfUserInfo {
    pub Username: String,
    pub Nickname: String,
    pub Email: String,
    pub Password: String,
}
