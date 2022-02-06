lazy_static! {
    pub static ref HEART_BEAT: String = String::from("heatbeat");
    pub static ref PONG: String = String::from("pong");
    pub static ref RUST_CHANNEL: String = String::from("channel");
    pub static ref KAFKA: String = String::from("kafka");
    pub static ref BROADCAST_CHANNEL_SIZE: usize = 32;
    pub static ref MPSC_CHANNEL_SIZE: usize = 32;
    pub static ref SIGNAL_CHANNEL_SIZE: usize = 4;
    pub static ref INVALID_ID: i32 = 0;
}

pub enum Signals {
    KillTask = 0,
}
