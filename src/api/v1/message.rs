use crate::common::date_format::my_date_format;
use crate::internal::model;
use axum::Json;
use chrono::{DateTime, Local};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::warn;
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct MessageRequest {
    pub MessageType: i32,
    pub Uuid: String,
    pub FriendUsername: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseMsg {
    pub code: i32,
    pub msg: String,
    pub data: Option<Value>,
}

impl ResponseMsg {
    pub fn success_msg(data: Value) -> (StatusCode, Json<Value>) {
        (
            StatusCode::OK,
            Json(
                serde_json::to_value(ResponseMsg {
                    code: 0,
                    msg: "SUCCESS".to_owned(),
                    data: Some(data),
                })
                .unwrap(),
            ),
        )
    }

    pub fn failed_msg(msg: String) -> (StatusCode, Json<Value>) {
        warn!("{}", msg);
        (
            StatusCode::OK,
            Json(
                serde_json::to_value(ResponseMsg {
                    code: -1,
                    msg,
                    data: None,
                })
                .unwrap(),
            ),
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GroupResponse {
    pub uuid: String,
    pub group_id: i32,
    #[serde(with = "my_date_format")]
    pub created_at: DateTime<Local>,
    pub name: String,
    pub notice: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResponse {
    pub user: model::user::User,
    pub group: model::group::Group,
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct MessageResponse {
    pub id: u64,
    pub from_user_id: i32,
    pub to_user_id: i32,
    pub content: String,
    pub content_type: i16,
    #[serde(with = "my_date_format")]
    pub created_at: DateTime<Local>,
    pub from_username: String,
    pub to_username: String,
    pub avatar: String,
    pub url: String,
}
