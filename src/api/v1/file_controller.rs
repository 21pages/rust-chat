use std::{ops::Add, sync::Arc};

use crate::{
    common::{constant, utils},
    internal::{service::user_service, state::AppState},
};
use axum::{
    extract::{ContentLengthLimit, Extension, Multipart, Path},
    Json,
};
use bytes::Bytes;
use http::StatusCode;
use serde_json::Value;
use std::env;
use tokio::sync::Mutex;
use tracing::info;
use uuid::Uuid;

use crate::api::v1::message::ResponseMsg;

pub async fn get(Path(filename): Path<String>) -> (StatusCode, Vec<u8>) {
    info!("get file:{:?}", filename);
    let path = env::var(&*constant::ENV_KEY_STATIC_FILEPATH)
        .unwrap()
        .add(filename.as_str());
    info!("get file path:{}", path);
    match utils::read_file(&path).await {
        Ok(v) => (StatusCode::OK, v),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, vec![]),
    }
}

/*
name:"uuid"
bytes:36

name:"file"
file_name:"QQ浏览器截图20190217213317.png"
content_type:"image/png"
bytes:173186
*/
pub async fn upload(
    ContentLengthLimit(mut multipart): ContentLengthLimit<
        Multipart,
        {
            250 * 1024 * 1024 /* 250mb */
        },
    >,
    Extension(state): Extension<Arc<Mutex<AppState>>>,
) -> (StatusCode, Json<Value>) {
    let mut user_uuid: Option<String> = None;
    let mut file_name: Option<String> = None;
    let mut file_bytes = Bytes::new();
    while let Some(field) = multipart.next_field().await.unwrap() {
        if let Some(name) = field.name() {
            info!("name:{:?}", name);
            if name == "uuid" {
                if let Ok(bytes) = field.bytes().await {
                    user_uuid = Some(String::from_utf8_lossy(&bytes.to_vec()).to_string());
                    info!("file uuid:{:?}", user_uuid);
                }
            } else if name == "file" {
                if let Some(v) = field.file_name() {
                    info!("file_name:{:?}", v);
                    file_name = Some(v.to_owned());
                }
                if let Ok(v) = field.bytes().await {
                    info!("file bytes len:{:?}", v.len());
                    file_bytes = v;
                }
            }
        }
    }
    if file_name.is_some() || user_uuid.is_some() || file_bytes.len() != 0 {
        let file_name = file_name.unwrap();
        let user_uuid = user_uuid.unwrap();
        if let Some(suffix_index) = file_name.rfind('.') {
            let new_name = Uuid::new_v4().to_string() + &file_name[suffix_index..];
            utils::write_file(
                &(env::var(&*constant::ENV_KEY_STATIC_FILEPATH).unwrap() + &new_name),
                &file_bytes,
            )
            .await
            .unwrap();
            user_service::modify_user_avatar(&new_name, &user_uuid, &state.lock().await.db)
                .await
                .unwrap();
            return ResponseMsg::success_msg(Value::String(new_name.clone()));
        }
    }
    ResponseMsg::failed_msg("upload failed".to_string())
}
