use std::path::PathBuf;

use thiserror::Error;

/// Errors returned by [`crate::Result`]. Parsing, file, and template errors only — HTTP/Rhai
/// execution errors live in `nd-engine::Error` (nd-core has no I/O runtime).
#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Plain(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("failed to parse YAML {path}: {source}")]
    ParseYaml {
        path: PathBuf,
        #[source]
        source: serde_yaml::Error,
    },

    #[error("failed to parse JSON {path}: {source}")]
    ParseJson {
        path: PathBuf,
        #[source]
        source: serde_json::Error,
    },

    #[error("failed to parse OpenAPI document {path}: {message}")]
    ParseOpenApi { path: PathBuf, message: String },

    #[error("unsupported file format (use .json, .yaml, or .yml): {0}")]
    UnsupportedFormat(PathBuf),

    #[error("operation not found: {method} {path} in {doc}")]
    OperationNotFound {
        doc: PathBuf,
        path: String,
        method: String,
    },

    /// A `${VAR}` reference had no value in [`crate::env::RuntimeEnv`] or the process environment.
    #[error("missing environment variable '{0}' in template")]
    MissingTemplateVar(String),

    /// Unknown `${!name}` dynamic template function (see `env::dynamic`).
    #[error("unknown dynamic template '{0}'")]
    UnknownDynamicTemplate(String),

    #[error("invalid request: {0}")]
    InvalidRequest(String),

    #[error("failed to read env file {path}: {source}")]
    EnvFileRead {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("invalid env file {path} line {line}: {message}")]
    EnvFileParse {
        path: PathBuf,
        line: usize,
        message: String,
    },

    #[error("No runtime persist file: {message}")]
    NoRuntimePersistFile { message: String },

    #[error("invalid runtime persist file {path}: {message}")]
    InvalidRuntimePersistFile { path: PathBuf, message: String },
}

/// Convenient alias used across this crate.
pub type Result<T> = std::result::Result<T, Error>;
