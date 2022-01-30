use axum::{
    routing::{get, post, put},
    Router,
};
use http::Uri;

pub fn new() -> Router {
    let app = Router::new()
        .route("/user", put(handler).get(handler))
        .route("/user/:uuid", get(handler))
        .route("/user/name", get(handler))
        .route("/user/register", post(handler))
        .route("/user/login", post(handler))
        .route("/friend", post(handler))
        .route("/message", get(handler))
        .route("/file/:fileName", get(handler))
        .route("/file", post(handler))
        .route("/group/:uuid", get(handler).post(handler))
        .route("/group/join/:userUuid/:groupUuid", post(handler))
        .route("/group/user/:uuid", get(handler))
        .route("/socket.io", get(handler));
    app
}

async fn handler(uri: Uri) -> String {
    format!("Hi from {}", uri.path()).to_string()
}
