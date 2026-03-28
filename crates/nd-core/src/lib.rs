//! Core library for **nativedoctor**: load request definitions (JSON/YAML), expand `${VAR}` templates,
//! run HTTP requests, and execute optional Rhai post-scripts.
//!
//! # Environment
//! [`RuntimeEnv`] seeds a writable map from the process environment. Lookups use that map first,
//! then fall back to live `std::env::var` for keys not present in the map (e.g. new vars after startup).
//!
//! # Rhai post-scripts
//! Scripts run in a locked-down Rhai engine. See [`rhai_host`] for the full API table and
//! [`rhai_host::run_post_script`].
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
pub mod rhai_host;
pub mod sequence;
mod template;

pub use discover::list_request_paths;
pub use env::RuntimeEnv;
pub use error::{Error, Result};
pub use execute::{
    execute_request_file, execute_request_with_env, format_prepared_request, prepare_request_file,
    prepare_request_with_env, ExecutionResult, OutcomePolicy, PreparedRequest, RunOptions,
};
pub use sequence::{execute_sequence, load_sequence_file, SequenceResult, StepSummary};
pub use load::{load_request_file, resolve_post_script};
pub use model::{HttpRequestSpec, RequestBody, RequestFile, SequenceFile, SequenceStep};
pub use template::{expand_json_value, expand_string};
