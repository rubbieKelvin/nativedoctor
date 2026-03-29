//! Core library for **nativedoctor**: load request definitions (JSON/YAML), expand `${VAR}` templates,
//! run HTTP requests, and execute optional Rhai post-scripts.
//!
//! # Internal logging
//! The crate emits [`tracing`] events (e.g. `debug!` when loading files, sending HTTP, running Rhai).
//! Consumers should install a [`tracing_subscriber`](https://docs.rs/tracing-subscriber) or use the
//! **nativedoctor** CLI, which sets `RUST_LOG` or defaults to `nd_core=debug` when `--verbose` is passed.
//!
//! # Environment
//! [`RuntimeEnv`] seeds a writable map from the process environment. Lookups use that map first,
//! then fall back to live `std::env::var` for keys not present in the map (e.g. new vars after startup).
//!
//! # Rhai post-scripts
//! Scripts run in a locked-down Rhai engine. See [`rhai::host`] for the built-in API and
//! [`run_post_script`]. `log(level, message)` emits [`tracing`] (CLI: `--verbose`); add a [`Logger`]
//! argument to [`run_post_script`] to capture the same lines in memory.
//!
//! # Sequences
//! Schema: [`SequenceFile`] / [`SequenceStep`]. Runner: [`sequence::execute_sequence`]
//! uses one shared [`RuntimeEnv`]. Step outcomes follow [`execute::OutcomePolicy::SequenceStep`].

mod discover;
mod env;
mod error;
mod execute;
mod load;
mod model;
pub mod rhai;
pub mod sequence;

pub use rhai::host::run_post_script;
mod template;

pub use discover::list_request_paths;
pub use env::RuntimeEnv;
pub use error::{Error, Result};
pub use execute::{
    execute_request_file, execute_request_with_env, format_prepared_request, prepare_request_file,
    prepare_request_with_env, ExecutionResult, OutcomePolicy, PreparedRequest, RunOptions,
};
pub use load::{load_request_file, resolve_post_script};
pub use model::{
    content_type_for_body_kind, request_file_json_schema, HttpRequestSpec, RequestBody,
    RequestBodyKind, RequestBodyStructured, RequestFile, SequenceFile, SequenceStep,
};
pub use sequence::{execute_sequence, load_sequence_file, SequenceResult, StepSummary};
pub use template::{expand_json_value, expand_string};
