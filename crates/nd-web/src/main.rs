//! Optional dev binary (`cargo run -p nd-web`): same defaults as `nativedoctor web`.

use std::net::SocketAddr;
use std::path::PathBuf;

fn main() -> Result<(), String> {
    let bind: SocketAddr = "127.0.0.1:8080"
        .parse()
        .map_err(|e| format!("bind address: {e}"))?;
    nd_web::run_web(PathBuf::from("."), bind, false, vec![], false)
}
