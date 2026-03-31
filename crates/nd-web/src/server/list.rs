//! List workspace files (requests, sequences, unknown).

use dioxus::fullstack::ServerFnError;
use dioxus::prelude::*;

use crate::config;

use super::types::CatalogEntry;

#[server]
pub async fn list_catalog() -> Result<Vec<CatalogEntry>, ServerFnError> {
    let cfg = config::web_config().ok_or_else(|| ServerFnError::new("workspace not configured"))?;
    let rows = nd_core::list_workspace_catalog(cfg.root.as_path())
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    let mut out = Vec::with_capacity(rows.len());
    for (path, kind) in rows {
        let name = path
            .file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_default();
        if name.is_empty() {
            continue;
        }
        out.push(CatalogEntry { name, kind });
    }
    Ok(out)
}
