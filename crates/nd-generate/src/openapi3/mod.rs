//! OpenAPI **3.0.x** → nativedoctor request files (`openapiv3`).
//!
//! Other document sources (e.g. other API description formats) can live as sibling modules under
//! `src/` later; this module keeps all OpenAPI-specific parsing, mapping, and file emission.

mod build;
mod fs;
mod load;

pub use build::{file_stem, operation_to_request_file, path_to_url_template, unique_stem};
pub use fs::{write_all_operations, write_request_file, OutputFormat};
pub use load::load_openapi;

use std::path::Path;

use crate::error::Result;
use crate::GenerateReport;

/// Read OpenAPI from `input` and write one request file per operation into `out_dir`.
pub(crate) fn generate_from_path(
    input: &Path,
    out_dir: &Path,
    format: OutputFormat,
) -> Result<GenerateReport> {
    let api = load::load_openapi(input)?;
    let files_written = fs::write_all_operations(&api, out_dir, format)?;
    Ok(GenerateReport { files_written })
}
