use crate::{
    api::v1::message::{MessageRequest, MessageResponse},
    internal::model::{message::Message, user::User},
};
use anyhow::Result;
use sqlx::MySqlPool;

pub async fn get_message(
    request: MessageRequest,
    pool: &MySqlPool,
) -> Result<Vec<MessageResponse>> {
    let current_user = User::get_by_uuid(&request.Uuid, pool).await?;
    let friend_user = User::get_by_username(&request.FriendUsername, pool).await?;
    Message::get_user_message(&current_user, &friend_user, pool).await
}
