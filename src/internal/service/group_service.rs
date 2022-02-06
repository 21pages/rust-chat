use crate::{
    api::v1::{
        infos::{GroupSave, GroupUserInfo},
        message::GroupResponse,
    },
    internal::model::{group::Group, group_member::GroupMember, user::User},
};
use anyhow::Result;
use sqlx::{self, MySqlPool};
use uuid::Uuid;

pub async fn get_uesr_groups(uuid: String, pool: &MySqlPool) -> Result<Vec<GroupResponse>> {
    let user = User::get_by_uuid(&uuid, pool).await?;
    Group::get_user_groups(user.id, pool).await
}

pub async fn save_group(user_uuid: &str, group_save: &GroupSave, pool: &MySqlPool) -> Result<()> {
    let user = User::get_by_uuid(&user_uuid, pool).await?;

    //insert group if not exist
    let mut group = Group {
        user_id: user.id,
        uuid: Uuid::new_v4().to_string(),
        name: group_save.name.clone(),
        created_at: chrono::offset::Local::now(),
        ..Default::default()
    };
    if !Group::exist_by_name(&group.name, pool).await? {
        group.insert(pool).await?;
    }

    //insert group member if not exist
    group = Group::get_by_name(&group.name, pool).await?;
    if !GroupMember::exist_by_user_id_group_id(group.user_id, group.id, pool).await? {
        let group_member = GroupMember {
            user_id: user.id,
            group_id: group.id,
            nickname: user.nickname,
            mute: 0,
            ..Default::default()
        };
        group_member.insert(pool).await?;
    }

    Ok(())
}

pub async fn join_group(user_uuid: &str, group_uuid: &str, pool: &MySqlPool) -> Result<()> {
    let user = User::get_by_uuid(&user_uuid, pool).await?;
    let group = Group::get_by_uuid(group_uuid, pool).await?;
    if GroupMember::exist_by_user_id_group_id(user.id, group.id, pool).await? {
        return Err(anyhow::anyhow!("joined already"));
    }

    let mut nickname = user.nickname.clone();
    if nickname == "".to_string() {
        nickname = user.username.clone();
    }
    let group_member = GroupMember {
        user_id: user.id,
        group_id: group.id,
        nickname,
        mute: 0,
        ..Default::default()
    };
    group_member.insert(pool).await?;
    Ok(())
}

pub async fn get_group_users(group_uuid: &str, pool: &MySqlPool) -> Result<Vec<GroupUserInfo>> {
    let group = Group::get_by_uuid(group_uuid, pool).await?;
    let user_infos = Group::get_users(group.id, pool).await?;
    Ok(user_infos)
}
