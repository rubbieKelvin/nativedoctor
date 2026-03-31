//! HTTP execution: expand templates, build a [`reqwest::Client`], send, then optional Rhai post-script.

pub mod client;
pub mod format;
pub mod prepare;
pub mod run;
pub mod types;
