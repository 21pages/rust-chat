use crate::{
    api::v1::message::{GroupMessageResponse, MessageRequest, UserMessageResponse},
    internal::model::{group::Group, message::Message, user::User},
};
use anyhow::Result;
use sqlx::MySqlPool;

pub async fn get_user_message(
    request: MessageRequest,
    pool: &MySqlPool,
) -> Result<Vec<UserMessageResponse>> {
    let current_user = User::get_by_uuid(&request.Uuid, pool).await?;
    let friend_user = User::get_by_username(&request.FriendUsername, pool).await?;
    Message::get_user_message(&current_user, &friend_user, pool).await
}

pub async fn get_group_message(uuid: &str, pool: &MySqlPool) -> Result<Vec<GroupMessageResponse>> {
    let group = Group::get_by_uuid(uuid, pool).await?;
    let messages = Message::get_group_message(group.id, pool).await?;
    Ok(messages)
}
