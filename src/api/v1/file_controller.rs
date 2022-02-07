use std::ops::Add;

use crate::common::constant;
use axum::{
    extract::{ContentLengthLimit, Multipart, Path},
    Json,
};
use http::StatusCode;
use serde_json::Value;
use std::env;
use tokio::{
    fs::File,
    io::{AsyncReadExt, BufReader},
};
use tracing::{error, info};

use crate::api::v1::message::ResponseMsg;

pub async fn get(Path(filename): Path<String>) -> (StatusCode, Vec<u8>) {
    info!("get file:{:?}", filename);
    let path = env::var(&*constant::ENV_KEY_STATIC_FILEPATH)
        .unwrap()
        .add(filename.as_str());
    match File::open(&path).await {
        Ok(f) => {
            let mut reader = BufReader::new(f);
            let mut buf = Vec::new();
            match reader.read_to_end(&mut buf).await {
                Ok(_) => (StatusCode::OK, buf),
                Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, vec![]),
            }
        }
        Err(e) => {
            error!("failed to open file {}:{:?}", path, e);
            (StatusCode::INTERNAL_SERVER_ERROR, vec![])
        }
    }
}

/*
doesn't work because axum doesn't support 'x-requested-with' yet
*/
pub async fn upload(
    ContentLengthLimit(mut multipart): ContentLengthLimit<
        Multipart,
        {
            250 * 1024 * 1024 /* 250mb */
        },
    >,
) -> (StatusCode, Json<Value>) {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let file_name = field.file_name().unwrap().to_string();
        let content_type = field.content_type().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        println!(
            "Length of `{}` (`{}`: `{}`) is {} bytes",
            name,
            file_name,
            content_type,
            data.len()
        );
    }
    ResponseMsg::failed_msg("todo".to_string())
}
