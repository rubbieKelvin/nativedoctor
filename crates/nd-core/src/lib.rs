//! Core library for **nativedoctor**: load request definitions (JSON/YAML), expand `${VAR}` and `${!name}` templates,
//! run HTTP requests, and execute optional Rhai post-scripts.

pub mod discover;
pub mod env;
pub mod error;
pub mod execute;
pub mod model;
pub mod rhai;
pub mod utils;
