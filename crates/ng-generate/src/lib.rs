//! Generate nativedoctor request files from **OpenAPI 3.0.x** specs (`openapiv3` parser).
//!
//! OpenAPI **3.1** is rejected with a clear error until a dedicated code path exists.
//!
//! YAML output quotes `request.url` when it contains `${…}` so YAML 1.1 does not treat `$` as an
//! alias. JSON output is unaffected.

mod build_request;
mod error;
mod fs_write;
mod load;

pub use build_request::path_to_url_template;
pub use error::{Error, Result};
pub use fs_write::OutputFormat;

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
///
/// File names derive from `operationId` or method + path. OpenAPI 3.1+ and unsupported `$ref` forms return [`Error`].
pub fn generate_from_openapi_path(
    input: impl AsRef<Path>,
    out_dir: impl AsRef<Path>,
    options: GenerateOptions,
) -> Result<GenerateReport> {
    let api = load::load_openapi(input.as_ref())?;
    let files_written = fs_write::write_all_operations(&api, out_dir.as_ref(), options.format)?;
    Ok(GenerateReport { files_written })
}
