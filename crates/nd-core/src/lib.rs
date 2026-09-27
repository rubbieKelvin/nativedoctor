//! Core data model for **nativedoctor**: OpenAPI-backed requests, the workspace manifest, and
//! environment/template expansion. No I/O runtime (no `reqwest`, no `tokio`) — HTTP execution and
//! the Rhai runtime live in `nd-engine`.

pub mod discover;
pub mod env;
pub mod error;
pub mod model;
pub mod utils;
pub mod workspace;

pub use error::{Error, Result};
pub use workspace::Workspace;
