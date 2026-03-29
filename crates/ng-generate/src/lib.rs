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
    pub format: OutputFormat,
}

impl Default for OutputFormat {
    fn default() -> Self {
        Self::Yaml
    }
}

/// Summary of a successful generation run.
#[derive(Debug, Clone)]
pub struct GenerateReport {
    pub files_written: Vec<std::path::PathBuf>,
}

/// Read OpenAPI from `input`, emit one request file per operation into `out_dir`.
pub fn generate_from_openapi_path(
    input: impl AsRef<Path>,
    out_dir: impl AsRef<Path>,
    options: GenerateOptions,
) -> Result<GenerateReport> {
    let api = load::load_openapi(input.as_ref())?;
    let files_written = fs_write::write_all_operations(&api, out_dir.as_ref(), options.format)?;
    Ok(GenerateReport { files_written })
}
