//! HTTP execution: expand templates, build a [`reqwest::Client`], send, then optional Rhai post-script.

mod client;
mod format;
mod post_script;
mod prepare;
mod run;
mod types;

pub use format::format_prepared_request;
pub use run::{
    execute_request_post_script, execute_request_with_env, prepare_request_file,
    prepare_request_with_env,
};
pub use types::{ExecutionResult, OutcomePolicy, PreparedRequest, RunOptions};
