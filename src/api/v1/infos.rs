use serde::Deserialize;
use sqlx;

#[derive(Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct FriendUserInfo {
    pub uuid: String,
    pub username: String,
    pub avatar: String,
}
