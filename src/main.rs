#[macro_use]
extern crate lazy_static;

use common::shutdown::shutdown_signal;
use internal::router;
use std::net::SocketAddr;
use tracing::info;

mod api;
mod common;
mod config;
mod internal;

lazy_static! {
    pub static ref CONFIG: config::TomlConfig = config::TomlConfig::init().unwrap();
}

#[tokio::main]
async fn main() {
    common::log::init();
    let addr = SocketAddr::from(([127, 0, 0, 1], 8888));
    info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(router::new().await.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}
