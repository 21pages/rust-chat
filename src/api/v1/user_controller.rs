use std::{collections::HashMap, sync::Arc};

use super::{
    infos::ModiyfUserInfo,
    message::{self, FriendRequest},
};
use crate::internal::{model::user::User, service::user_service, state::AppState};
use axum::{
    extract::{Extension, Path, Query},
    Json,
};
use http::StatusCode;
use serde::Deserialize;
use serde_json::Value;
use tokio::sync::Mutex;
use tracing::info;

#[derive(Debug, Deserialize)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}

pub async fn login(
    Json(login_user): Json<LoginUser>,
    Extension(state): Extension<Arc<Mutex<AppState>>>,
) -> (StatusCode, Json<Value>) {
    info!("user login:{:?}", login_user);
    let mut user = User::default();
    user.username = login_user.username;
    user.password = login_user.password;
    if let Err(e) = user_service::login(&mut user, &state.clone().lock().await.db).await {
        message::ResponseMsg::failed_msg(format!("Login failed:{:?}", e).to_string())
    } else {
        message::ResponseMsg::success_msg(user.to_json_value())
    }
}

#[derive(Debug, Deserialize)]
pub struct RegisterUser {
    pub username: String,
    pub password: String,
    pub email: String,
    pub nickname: String,
}

pub async fn register(
    Json(register_user): Json<RegisterUser>,
    Extension(state): Extension<Arc<Mutex<AppState>>>,
) -> (StatusCode, Json<Value>) {
    info!("user register:{:?}", register_user);
    let mut user = User::default();
    user.username = register_user.username;
    user.password = register_user.password;
    user.email = register_user.email;
    user.nickname = register_user.nickname;
    if let Err(e) = user_service::register(&mut user, &state.clone().lock().await.db).await {
        message::ResponseMsg::failed_msg(format!("Register failed:{:?}", e).to_string())
    } else {
        message::ResponseMsg::success_msg(user.to_json_value())
    }
}

pub async fn get_user_details(
    Path(uuid): Path<String>,
    Extension(state): Extension<Arc<Mutex<AppState>>>,
) -> (StatusCode, Json<Value>) {
    info!("get user details:{:?}", uuid);
    match user_service::get_user_details(uuid, &state.clone().lock().await.db).await {
        Ok(user) => message::ResponseMsg::success_msg(user.to_json_value()),
        Err(e) => {
            message::ResponseMsg::failed_msg(format!("Get user details failed:{:?}", e).to_string())
        }
    }
}

pub async fn get_user_list(
    Query(params): Query<HashMap<String, String>>,
    Extension(state): Extension<Arc<Mutex<AppState>>>,
) -> (StatusCode, Json<Value>) {
    let uuid = params.get("uuid").unwrap().to_string();
    info!("get user list:{:?}", uuid);
    match user_service::get_user_list(uuid, &state.clone().lock().await.db).await {
        Ok(users) => message::ResponseMsg::success_msg(serde_json::to_value(users).unwrap()),
        Err(e) => {
            message::ResponseMsg::failed_msg(format!("Get user list failed:{:?}", e).to_string())
        }
    }
}

pub async fn modify_user_info(
    Json(info): Json<ModiyfUserInfo>,
    Extension(state): Extension<Arc<Mutex<AppState>>>,
) -> (StatusCode, Json<Value>) {
    info!("modify user info:{:?}", info);
    match user_service::modify_user_info(info, &state.clone().lock().await.db).await {
        Ok(users) => message::ResponseMsg::success_msg(serde_json::to_value(users).unwrap()),
        Err(e) => {
            message::ResponseMsg::failed_msg(format!("modify user info failed:{:?}", e).to_string())
        }
    }
}

pub async fn get_user_or_group_by_name(
    Query(params): Query<HashMap<String, String>>,
    Extension(state): Extension<Arc<Mutex<AppState>>>,
) -> (StatusCode, Json<Value>) {
    let name = params.get("name").unwrap().to_string();
    info!("get user or group by name:{}", name);
    match user_service::get_user_or_group_by_name(&name, &state.clone().lock().await.db).await {
        Ok(v) => message::ResponseMsg::success_msg(serde_json::to_value(v).unwrap()),
        Err(e) => message::ResponseMsg::failed_msg(
            format!("Get user or group by name failed:{:?}", e).to_string(),
        ),
    }
}

pub async fn add_friend(
    Json(request): Json<FriendRequest>,
    Extension(state): Extension<Arc<Mutex<AppState>>>,
) -> (StatusCode, Json<Value>) {
    info!("add friend:{:?}", request);
    match user_service::add_friend(request, &state.clone().lock().await.db).await {
        Ok(users) => message::ResponseMsg::success_msg(serde_json::to_value(users).unwrap()),
        Err(e) => {
            message::ResponseMsg::failed_msg(format!("add friend failed:{:?}", e).to_string())
        }
    }
}
