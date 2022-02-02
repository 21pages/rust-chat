use super::message;
use crate::internal::{model::user::User, service::user_service, state::AppState};
use axum::{extract::Extension, Json};
use http::StatusCode;
use serde::Deserialize;
use serde_json::Value;
use tracing::info;

#[derive(Debug, Deserialize)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}

pub async fn login(
    Json(login_user): Json<LoginUser>,
    Extension(state): Extension<AppState>,
) -> (StatusCode, Json<Value>) {
    info!("user login:{:?}", login_user);
    let mut user = User::default();
    user.username = login_user.username;
    user.password = login_user.password;
    if let Ok(_) = user_service::login(&mut user, state.db.clone()).await {
        (
            StatusCode::OK,
            message::ResponseMsg::success_msg(user.to_json_value()),
        )
    } else {
        (
            StatusCode::OK,
            message::ResponseMsg::failed_msg("Login failed".to_owned()),
        )
    }
}
