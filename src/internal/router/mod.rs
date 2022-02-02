use crate::api::v1;
use crate::internal::state::AppState;
use axum::{
    extract::Extension,
    routing::{get, post, put},
    AddExtensionLayer, Router,
};
use http::{
    header::{
        ACCEPT, ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_ORIGIN, AUTHORIZATION,
        CACHE_CONTROL, CONTENT_LANGUAGE, CONTENT_LENGTH, CONTENT_TYPE, ORIGIN,
    },
    Method, Uri,
};
use tower_http::{
    cors::{any, CorsLayer},
    trace::TraceLayer,
};

pub async fn new() -> Router {
    let state = AppState::new().await.unwrap();

    let app = Router::new()
        .route(
            "/user",
            put(handler).get(v1::user_controller::get_user_list),
        )
        .route("/user/:uuid", get(v1::user_controller::get_user_details))
        .route("/user/name", get(handler))
        .route("/user/register", post(handler))
        .route("/user/login", post(v1::user_controller::login))
        .route("/friend", post(handler))
        .route("/message", get(handler))
        .route("/file/:fileName", get(handler))
        .route("/file", post(handler))
        .route("/group/:uuid", get(handler).post(handler))
        .route("/group/join/:userUuid/:groupUuid", post(handler))
        .route("/group/user/:uuid", get(handler))
        .route("/socket.io", get(handler))
        .layer(TraceLayer::new_for_http())
        .layer(AddExtensionLayer::new(state))
        .layer(cors());
    app
}

async fn handler(Extension(_state): Extension<AppState>, uri: Uri) -> String {
    format!("Hi from {:?}", uri)
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
        ])
        .allow_headers(vec![ORIGIN, CONTENT_TYPE, ACCEPT, AUTHORIZATION])
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
