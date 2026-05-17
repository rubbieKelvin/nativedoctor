//! Generate nativedoctor request files from external API descriptions.
//!
//! Today the supported source is **OpenAPI 3.0.x** via [`openapi3`]. Additional sources can add
//! their own top-level modules alongside it.
//!
//! OpenAPI **3.1** is rejected with a clear error until a dedicated code path exists.
//!
//! YAML output quotes `request.url` when it contains `${…}` so YAML 1.1 does not treat `$` as an
//! alias. JSON output is unaffected.

pub mod openapi3;

pub use openapi3::OutputFormat;

use std::path::Path;

/// Options for [`generate_from_openapi_path`].
#[derive(Debug, Clone, Copy, Default)]
pub struct GenerateOptions {
    /// YAML or JSON output for each generated request file.
    pub format: OutputFormat,
}

/// Summary of a successful generation run.
#[derive(Debug, Clone)]
pub struct GenerateReport {
    /// Absolute or relative paths of files created under `out_dir`.
    pub files_written: Vec<std::path::PathBuf>,
}

/// Read OpenAPI 3.0.x from `input`, then write one nativedoctor request file per HTTP operation into `out_dir`.
/// File names derive from `operationId` or method + path. OpenAPI 3.1+ and unsupported `$ref` forms return [`Error`].
pub fn generate_from_openapi_path(
    input: impl AsRef<Path>,
    out_dir: impl AsRef<Path>,
    options: GenerateOptions,
) -> Result<GenerateReport, openapi3::error::Error> {
    openapi3::generate_from_path(input.as_ref(), out_dir.as_ref(), options.format)
}

pub use openapi3::path_to_url_template;
