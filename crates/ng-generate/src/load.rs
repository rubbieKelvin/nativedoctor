use std::path::Path;

use openapiv3::OpenAPI;

use crate::error::{Error, Result};

/// Read and deserialize OpenAPI 3.0.x from `.json`, `.yaml`, or `.yml` (extension-based).
pub fn load_openapi(path: &Path) -> Result<OpenAPI> {
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
    Ok(api)
}

fn check_openapi_version(api: &OpenAPI) -> Result<()> {
    let v = api.openapi.trim();
    if v == "3.0" || v.starts_with("3.0.") {
        return Ok(());
    }
    Err(Error::UnsupportedOpenApiVersion(v.to_string()))
}
