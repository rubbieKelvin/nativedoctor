use thiserror::Error;

/// Errors from HTTP execution and the Rhai runtime. Wraps [`nd_core::Error`] for parsing/template
/// errors that surface while preparing a request.
#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Core(#[from] nd_core::Error),

    #[error("invalid request: {0}")]
    InvalidRequest(String),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Rhai script error: {0}")]
    Rhai(String),

    #[error("post_script not found: {0}")]
    PostScriptNotFound(std::path::PathBuf),
}

pub type Result<T> = std::result::Result<T, Error>;
