use crate::api::v1::{
    file_controller, group_controller, message_controller, user_controller, ws_controller,
};
use crate::internal::state::AppState;
use axum::{
    http::header::HeaderName,
    routing::{get, post, put},
    AddExtensionLayer, Router,
};
use http::{
    header::{
        ACCEPT, ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_ORIGIN, AUTHORIZATION,
        CACHE_CONTROL, CONTENT_LANGUAGE, CONTENT_LENGTH, CONTENT_TYPE, ORIGIN,
    },
    Method,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::{
    cors::{any, CorsLayer},
    trace::TraceLayer,
};

pub async fn new() -> Router {
    //mutex for mutable, tokio::sync::Mutex for async
    let state = Arc::new(Mutex::new(AppState::new().await.unwrap()));

    let app = Router::new()
        .route(
            "/user",
            put(user_controller::modify_user_info).get(user_controller::get_user_list),
        )
        .route("/user/:uuid", get(user_controller::get_user_details))
        .route(
            "/user/name",
            get(user_controller::get_user_or_group_by_name),
        )
        .route("/user/register", post(user_controller::register))
        .route("/user/login", post(user_controller::login))
        .route("/friend", post(user_controller::add_friend))
        .route("/message", get(message_controller::get_message))
        .route("/file/:fileName", get(file_controller::get))
        .route("/file", post(file_controller::upload))
        .route(
            "/group/:uuid",
            get(group_controller::get_user_group).post(group_controller::save_group),
        )
        .route(
            "/group/join/:userUuid/:groupUuid",
            post(group_controller::join_group),
        )
        .route("/group/user/:uuid", get(group_controller::get_group_users))
        .route("/socket.io", get(ws_controller::ws_handler))
        .layer(TraceLayer::new_for_http())
        .layer(AddExtensionLayer::new(state))
        .layer(cors());
    app
}

fn cors() -> CorsLayer {
    CorsLayer::new()
        // point out all allow!!
        .allow_methods(vec![
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::OPTIONS,
            Method::DELETE,
            Method::HEAD,
            Method::DELETE,
            Method::CONNECT,
            Method::PATCH,
            Method::TRACE,
        ])
        .allow_headers(vec![
            ORIGIN,
            CONTENT_TYPE,
            ACCEPT,
            AUTHORIZATION,
            HeaderName::from_static("x-requested-with"),
        ])
        .allow_credentials(true)
        .allow_origin(any())
        .expose_headers(vec![
            CONTENT_LENGTH,
            ACCESS_CONTROL_ALLOW_ORIGIN,
            ACCESS_CONTROL_ALLOW_HEADERS,
            CACHE_CONTROL,
            CONTENT_LANGUAGE,
            CONTENT_TYPE,
        ])
}
