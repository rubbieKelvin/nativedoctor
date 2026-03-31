//! Payload types for server functions.

use serde::{Deserialize, Serialize};

use nd_core::WorkspaceFileKind;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CatalogEntry {
    pub name: String,
    pub kind: WorkspaceFileKind,
}
