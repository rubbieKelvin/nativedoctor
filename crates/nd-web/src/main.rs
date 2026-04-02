//! Optional standalone binary for the web UI (primary entry is `nativedoctor web` from `nd-cli`).
//!
//! Initializes `tracing` from `RUST_LOG` (default `info`), then calls [`nd_web::run_web`] with minimal CLI parsing:
//! `--bind ADDR` and positional workspace directories (default `.`).

use std::net::SocketAddr;
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let mut args = std::env::args().skip(1).collect::<Vec<_>>();
    let bind: SocketAddr = if args.first().map(|s| s.as_str()) == Some("--bind") {
        args.remove(0);
        args.remove(0)
            .parse()
            .unwrap_or_else(|_| "127.0.0.1:8080".parse().unwrap())
    } else {
        "127.0.0.1:8080".parse()?
    };

    let roots: Vec<PathBuf> = if args.is_empty() {
        vec![PathBuf::from(".")]
    } else {
        args.into_iter().map(PathBuf::from).collect()
    };

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;

    rt.block_on(nd_web::run_web(nd_web::WebServerOptions {
        bind,
        roots,
        env_files: vec![],
        persistence_file: None,
        no_network_io: false,
    }))?;

    return Ok(());
}
