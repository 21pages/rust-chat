use crate::{
    api::v1::message::{GroupMessageResponse, MessageRequest, UserMessageResponse},
    internal::model::{group::Group, message::Message, user::User},
    protos::proto as pb,
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

pub async fn save_message(message: pb::Message, pool: &MySqlPool) -> Result<()> {
    let from_user = User::get_by_uuid(&message.from, pool).await?;
    let mut message_save = Message {
        from_user_id: from_user.id,
        content: message.content,
        message_type: message.message_type as i16,
        content_type: message.content_type as i16,
        url: message.url,
        ..Default::default()
    };
    if message.message_type == pb::MessageUserType::User as i32 {
        let to_user = User::get_by_uuid(&message.to, pool).await?;
        message_save.to_user_id = to_user.id;
    } else if message.message_type == pb::MessageUserType::Group as i32 {
        let to_group = Group::get_by_uuid(&message.to, pool).await?;
        message_save.to_user_id = to_group.id;
    } else {
        return Err(anyhow::anyhow!("unknown message to user type "));
    }
    message_save.insert(pool).await?;
    Ok(())
}
