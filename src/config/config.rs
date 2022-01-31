use serde::{Deserialize, Serialize};
use std::fs;
use toml;

impl TomlConfig {
    pub fn init() -> Result<TomlConfig, Box<dyn std::error::Error>> {
        let content = fs::read_to_string("config.toml")?;
        let toml_config = toml::from_str(content.as_str())?;
        Ok(toml_config)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TomlConfig {
    pub appname: String,
    pub mysql: MySQLConfig,
    pub log: LogConfig,
    pub static_path: PathConfig,
    pub msg_channel_type: MsgChannelType,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MySQLConfig {
    pub host: String,
    pub name: String,
    pub password: String,
    pub port: i32,
    pub table_prefix: String,
    pub user: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LogConfig {
    pub path: String,
    pub level: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PathConfig {
    pub filepath: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MsgChannelType {
    pub channel_type: String,
    pub kafka_hosts: String,
    pub kafka_topic: String,
}
