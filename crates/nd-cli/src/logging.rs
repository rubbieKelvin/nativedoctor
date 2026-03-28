//! Initialize [`tracing`] for the binary: logs go to **stderr** so stdout stays clean for bodies.

use tracing_subscriber::EnvFilter;

/// Install a global subscriber. Safe to call once; later calls are ignored (`try_init`).
///
/// - If **`RUST_LOG`** is set, it wins (standard `tracing-subscriber` env filter).
/// - Else if **`verbose`**, default filter is `nd_core=debug,warn` (internal detail + warnings).
/// - Else default is **`warn`** (only warnings and errors from any crate).
pub fn init(verbose: bool) {
    let fallback = if verbose {
        "nd_core=debug,warn"
    } else {
        "warn"
    };
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(fallback));

    let _ = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(true)
        .compact()
        .with_writer(std::io::stderr)
        .try_init();
}
