use crate::{internal::db, server::server::Server};
use anyhow::Result;
use sqlx::MySqlPool;

// #[derive(Clone)]
pub struct AppState {
    pub db: MySqlPool,
    pub server: Server,
}

impl AppState {
    pub async fn new() -> Result<Self> {
        let db = db::new().await;
        let server = Server::new();
        Ok(AppState { db, server })
    }
}
