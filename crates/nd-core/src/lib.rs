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

pub mod discover;
pub mod env;
pub mod error;
pub mod execute;
pub mod model;
pub mod rhai;
pub mod utils;
