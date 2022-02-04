use std::env;
use std::io::Result;
fn main() -> Result<()> {
    env::set_var("OUT_DIR", "src/protos");
    prost_build::compile_protos(&["src/protos/ws_message.proto"], &["src/protos"])?;
    Ok(())
}
