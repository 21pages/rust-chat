#[macro_use]
extern crate lazy_static;

use std::net::SocketAddr;
use tracing::info;

mod config;
mod router;
mod utils;

lazy_static! {
    pub static ref CONFIG: config::config::TomlConfig = config::config::TomlConfig::init().unwrap();
}

#[tokio::main]
async fn main() {
    utils::log::init();
    let addr = SocketAddr::from(([127, 0, 0, 1], 8888));
    info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(router::router::new().into_make_service())
        .await
        .unwrap();
}
