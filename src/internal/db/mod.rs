use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
use std::env;
use std::time::Duration;
use tracing::info;

pub async fn new() -> MySqlPool {
    let dsn = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    info!("mysql dsn:{}", dsn);
    MySqlPoolOptions::new()
        .max_connections(100)
        .min_connections(10)
        .connect_timeout(Duration::from_secs(1))
        .connect(&dsn)
        .await
        .unwrap()
}
