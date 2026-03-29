//! Initialize [`tracing`] for the binary: logs go to **stderr** so stdout stays clean for bodies.

use nd_constants::{CLI_TRACING_FILTER_QUIET, CLI_TRACING_FILTER_VERBOSE};
use tracing_subscriber::EnvFilter;

/// Install a global subscriber. Safe to call once; later calls are ignored (`try_init`).
///
/// - If **`RUST_LOG`** is set, it wins (standard `tracing-subscriber` env filter).
/// - Else if **`verbose`**, default filter matches [`CLI_TRACING_FILTER_VERBOSE`] (internal detail + warnings).
/// - Else default matches [`CLI_TRACING_FILTER_QUIET`] (only warnings and errors from any crate).
pub fn init(verbose: bool) {
    let fallback = if verbose {
        CLI_TRACING_FILTER_VERBOSE
    } else {
        CLI_TRACING_FILTER_QUIET
    };
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(fallback));

    let _ = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(true)
        .compact()
        .with_writer(std::io::stderr)
        .try_init();
}
