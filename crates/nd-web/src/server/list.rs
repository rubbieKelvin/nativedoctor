//! List workspace files (requests, sequences, unknown).

use dioxus::fullstack::ServerFnError;
use dioxus::prelude::*;

use crate::config;

use super::types::CatalogEntry;

#[server]
pub async fn list_catalog() -> Result<Vec<CatalogEntry>, ServerFnError> {
    let _cfg =
        config::web_config().ok_or_else(|| ServerFnError::new("workspace not configured"))?;
    todo!();
}
