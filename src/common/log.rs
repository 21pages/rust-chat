use std::env;
use tracing::{subscriber, Level};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

pub fn init() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "rust_chat=trace")
    }

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    subscriber::set_global_default(subscriber).unwrap();
}
