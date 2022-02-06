use std::sync::Arc;

use super::{infos::GroupSave, message::ResponseMsg};
use crate::internal::{service::group_service, state::AppState};
use axum::{
    extract::{Extension, Path},
    Json,
};
use http::StatusCode;
use serde::Deserialize;
use serde_json::Value;
use tokio::sync::Mutex;
use tracing::info;

pub async fn get_user_group(
    Path(uuid): Path<String>,
    Extension(state): Extension<Arc<Mutex<AppState>>>,
) -> (StatusCode, Json<Value>) {
    info!("get group:{:?}", uuid);
    match group_service::get_uesr_groups(uuid, &state.clone().lock().await.db).await {
        Ok(groups) => ResponseMsg::success_msg(serde_json::to_value(groups).unwrap()),
        Err(e) => ResponseMsg::failed_msg(format!("Get user group failed:{:?}", e).to_string()),
    }
}

pub async fn save_group(
    Path(user_uuid): Path<String>,
    Json(groupsave): Json<GroupSave>,
    Extension(state): Extension<Arc<Mutex<AppState>>>,
) -> (StatusCode, Json<Value>) {
    info!("save group:{:?} by user {}", groupsave, user_uuid);
    match group_service::save_group(&user_uuid, &groupsave, &state.clone().lock().await.db).await {
        Ok(v) => ResponseMsg::success_msg(serde_json::to_value(v).unwrap()),
        Err(e) => ResponseMsg::failed_msg(format!("save group failed:{:?}", e).to_string()),
    }
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct GroupJoinPath {
    pub userUuid: String,
    pub groupUuid: String,
}
pub async fn join_group(
    Path(path): Path<GroupJoinPath>,
    Extension(state): Extension<Arc<Mutex<AppState>>>,
) -> (StatusCode, Json<Value>) {
    info!("join group:{:?}", path);
    match group_service::join_group(
        &path.userUuid,
        &path.groupUuid,
        &state.clone().lock().await.db,
    )
    .await
    {
        Ok(v) => ResponseMsg::success_msg(serde_json::to_value(v).unwrap()),
        Err(e) => ResponseMsg::failed_msg(format!("join group failed:{:?}", e).to_string()),
    }
}

pub async fn get_group_users(
    Path(uuid): Path<String>,
    Extension(state): Extension<Arc<Mutex<AppState>>>,
) -> (StatusCode, Json<Value>) {
    info!("get group users:{:?}", uuid);
    match group_service::get_group_users(&uuid, &state.clone().lock().await.db).await {
        Ok(v) => ResponseMsg::success_msg(serde_json::to_value(v).unwrap()),
        Err(e) => ResponseMsg::failed_msg(format!("get group users failed:{:?}", e).to_string()),
    }
}
