pub fn init() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "rust_chat=trace")
    }
    tracing_subscriber::fmt::init();
}
