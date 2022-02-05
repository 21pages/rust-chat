use std::sync::Arc;

use axum::{
    extract::{Extension, Query},
    Json,
};
use http::StatusCode;
use serde_json::Value;
use tokio::sync::Mutex;
use tracing::trace;

use crate::{
    api::v1::message::ResponseMsg,
    internal::{service::message_service, state::AppState},
};

use super::message::MessageRequest;

pub async fn get_message(
    Query(request): Query<MessageRequest>,
    Extension(state): Extension<Arc<Mutex<AppState>>>,
) -> (StatusCode, Json<Value>) {
    trace!("message request:{:?}", request);
    match message_service::get_message(request, &state.lock().await.db).await {
        Ok(message) => ResponseMsg::success_msg(serde_json::to_value(message).unwrap()),
        Err(e) => ResponseMsg::failed_msg(format!("get message failed:{:?}", e)),
    }
}
