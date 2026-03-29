use std::path::PathBuf;

/// Failure modes for loading OpenAPI, validating version, or writing generated files.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unsupported OpenAPI version: {0} (only 3.0.x is supported in this release)")]
    UnsupportedOpenApiVersion(String),
    #[error("path item $ref not supported: {0}")]
    PathItemRef(String),
    #[error("parameter $ref not supported")]
    ParameterRef,
    #[error("request body $ref not supported")]
    RequestBodyRef,
    #[error("I/O error at {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to parse OpenAPI as JSON: {0}")]
    Json(#[from] serde_json::Error),
    #[error("failed to parse OpenAPI as YAML: {0}")]
    Yaml(#[from] serde_yaml::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
