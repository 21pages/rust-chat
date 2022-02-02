use crate::CONFIG;
use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
use std::time::Duration;
use tracing::info;

pub async fn new() -> MySqlPool {
    let mysql = &CONFIG.mysql;
    let username = &mysql.user;
    let password = &mysql.password;
    let host = &mysql.host;
    let port = mysql.port;
    let name = &mysql.name;

    let dsn = format!(
        "mysql://{username}:{password}@{host}:{port}/{name}",
        username = username,
        password = password,
        host = host,
        port = port,
        name = name,
    );
    info!("mysql dsn:{}", dsn);
    MySqlPoolOptions::new()
        .max_connections(100)
        .min_connections(10)
        .connect_timeout(Duration::from_secs(1))
        .connect(&dsn)
        .await
        .unwrap()
}
