lazy_static! {
    pub static ref HEART_BEAT: String = String::from("heatbeat");
    pub static ref PONG: String = String::from("pong");
    pub static ref BROADCAST_CHANNEL_SIZE: usize = 32;
    pub static ref MPSC_CHANNEL_SIZE: usize = 32;
    pub static ref SIGNAL_CHANNEL_SIZE: usize = 4;
    pub static ref INVALID_ID: i32 = 0;
    pub static ref ENV_KEY_DATABASE_URL: String = String::from("DATABASE_URL");
    pub static ref ENV_KEY_LOG_LEVLE: String = String::from("log_level");
    pub static ref ENV_KEY_LOG_PATH: String = String::from("log_path");
    pub static ref ENV_KEY_STATIC_FILEPATH: String = String::from("static_filepath");
    pub static ref ENV_KEY_MSG_TYPE: String = String::from("msg_type");
    pub static ref ENV_VAL_MSG_TYPE_CHANNEL: String = String::from("channel");
    pub static ref ENV_VAL_MSG_TYPE_KAFKA: String = String::from("kafka");
    pub static ref ENV_KEY_KAFKA_HOSTS: String = String::from("kafka_hosts");
    pub static ref ENV_KEY_KAFKA_TOPIC: String = String::from("kafka_topic");
}

pub enum Signals {
    KillTask = 0,
}
