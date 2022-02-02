use crate::internal::db;
use sqlx::MySqlPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<MySqlPool>,
}

impl AppState {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let db = db::new().await;
        Ok(AppState { db: Arc::new(db) })
    }
}
