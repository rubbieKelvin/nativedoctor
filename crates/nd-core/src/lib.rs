//! Core library for **nativedoctor**: load request definitions (JSON/YAML), expand `${VAR}` and `${!name}` templates,
//! run HTTP requests, and execute optional Rhai post-scripts.
//!
//! # Internal logging
//! The crate emits [`tracing`] events (e.g. `debug!` when loading files, sending HTTP, running Rhai).
//! Consumers should install a [`tracing_subscriber`](https://docs.rs/tracing-subscriber) or use the
//! **nativedoctor** CLI, which sets `RUST_LOG` or defaults to `nd_core=debug` when `--verbose` is passed.
//!
//! # Environment
//! [`RuntimeEnv`] holds a writable map. [`RuntimeEnv::from_process_env`] copies the process
//! environment; [`RuntimeEnv::isolated`] starts empty (no process fallback). Lookups use the map
//! first, then (when configured) live `std::env::var` for keys not present in the map. [`RuntimeEnv::merge_env_file`]
//! loads dotenv-style `KEY=value` files into the map. [`RuntimeEnv::merge_runtime_persist_dir`] loads
//! `runtime.nativedoctor.json` from a given directory (the CLI uses the **current working directory**).
//! [`RuntimeEnv::from_cli_options`] matches the CLI layering: optional `--no-default-system-env`, cwd
//! persist file, then each `--env` file in order.
//!
//! # Rhai post-scripts
//! Scripts run in a locked-down Rhai engine. See [`rhai::run`] for the built-in API and
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

pub use rhai::run_post_script;
mod template;

pub use discover::list_request_paths;
pub use env::RuntimeEnv;
pub use error::{Error, Result};
pub use execute::{
    execute_request_post_script, execute_request_with_env, format_prepared_request,
    prepare_request_file, prepare_request_with_env, ExecutionResult, OutcomePolicy,
    PreparedRequest, RunOptions,
};
pub use load::{load_request_file, normalize_path_lexical, resolve_post_script};
pub use model::{
    content_type_for_body_kind, request_file_json_schema, sequence_file_json_schema,
    with_root_schema_url, HttpRequestSpec, RequestBody, RequestBodyKind, RequestBodyStructured,
    RequestFile, SequenceFile, SequenceStep,
};
pub use sequence::{
    execute_sequence, load_sequence_file, sequence_step_iter, SequenceResult, StepSummary,
};
pub use template::{expand_hashmap_values, expand_json_value, expand_string};
