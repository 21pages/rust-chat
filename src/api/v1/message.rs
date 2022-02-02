use crate::common::date_format::my_date_format;
use crate::internal::model;
use axum::Json;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageRequest {
    #[serde(rename = "messageType")]
    message_type: i32,
    uuid: String,
    #[serde(rename = "friendUsername")]
    friend_username: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseMsg {
    code: i32,
    msg: String,
    data: Option<Value>,
}

impl ResponseMsg {
    pub fn success_msg(data: Value) -> Json<Value> {
        Json(
            serde_json::to_value(ResponseMsg {
                code: 0,
                msg: "SUCCESS".to_owned(),
                data: Some(data),
            })
            .unwrap(),
        )
    }

    pub fn failed_msg(msg: String) -> Json<Value> {
        Json(
            serde_json::to_value(ResponseMsg {
                code: -1,
                msg,
                data: None,
            })
            .unwrap(),
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GroupResponse {
    uuid: String,
    group_id: i32,
    #[serde(with = "my_date_format")]
    create_at: DateTime<Local>,
    name: String,
    notice: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResponse {
    user: model::user::User,
    group: model::group::Group,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MessageResponse {
    id: i32,
    from_user_id: i32,
    to_user_id: i32,
    content: String,
    content_type: i16,
    #[serde(with = "my_date_format")]
    create_at: DateTime<Local>,
    from_username: String,
    to_username: String,
    avatar: String,
    url: String,
}
