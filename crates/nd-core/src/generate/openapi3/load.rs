//! Read OpenAPI 3.0.x specs from `.json`, `.yaml`, or `.yml`.

use super::error::Error;
use openapiv3::OpenAPI;
use std::path::Path;

/// Read and deserialize OpenAPI 3.0.x from `.json`, `.yaml`, or `.yml` (extension-based).
pub fn load_openapi(path: &Path) -> Result<OpenAPI, Error> {
    let text = std::fs::read_to_string(path).map_err(|source| Error::Io {
        path: path.to_path_buf(),
        source,
    })?;

    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();

    let api: OpenAPI = match ext.as_str() {
        "yaml" | "yml" => serde_yaml::from_str(&text)?,
        _ => serde_json::from_str(&text)?,
    };

    check_openapi_version(&api)?;
    return Ok(api);
}

fn check_openapi_version(api: &OpenAPI) -> Result<(), Error> {
    let v = api.openapi.trim();
    if v == "3.0" || v.starts_with("3.0.") {
        return Ok(());
    }
    return Err(Error::UnsupportedOpenApiVersion(v.to_string()));
}
