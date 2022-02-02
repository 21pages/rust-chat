#[macro_use]
extern crate lazy_static;

use common::shutdown::shutdown_signal;
use dotenv::dotenv;
use internal::router;
use std::net::SocketAddr;
use tracing::info;

mod api;
mod common;
mod internal;

#[tokio::main]
async fn main() {
    dotenv().ok();
    common::log::init();
    let addr = SocketAddr::from(([127, 0, 0, 1], 8888));
    info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(router::new().await.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}
