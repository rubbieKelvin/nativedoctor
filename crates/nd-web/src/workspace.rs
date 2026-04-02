//! Aggregate discovery across one or more non-recursive directory roots.

use std::path::PathBuf;

use nd_core::discover::{
    list_request_paths, list_rhai_paths, partition_valid_request_paths, SkippedRequestFile,
};
use nd_core::error::{Error, Result as NdResult};
use tracing::debug;

use crate::path_sandbox::canonicalize_roots;

#[derive(Debug, Clone, serde::Serialize)]
pub struct RootInfo {
    pub index: usize,
    pub path: String,
    /// Short label (directory name) for UI when multiple roots.
    pub label: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct FileEntry {
    pub path: String,
    pub name: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct GroupedFiles {
    pub root_index: usize,
    pub root_label: String,
    pub entries: Vec<FileEntry>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct WorkspaceSnapshot {
    pub roots: Vec<RootInfo>,
    pub requests: Vec<GroupedFiles>,
    pub scripts: Vec<GroupedFiles>,
    pub skipped_requests: Vec<SkippedRequestFile>,
}

pub fn build_workspace(roots: &[PathBuf]) -> NdResult<WorkspaceSnapshot> {
    let canon =
        canonicalize_roots(roots).map_err(|e| Error::InvalidRequest(format!("workspace roots: {e}")))?;

    let root_infos: Vec<RootInfo> = canon
        .iter()
        .enumerate()
        .map(|(index, p)| RootInfo {
            index,
            path: p.to_string_lossy().to_string(),
            label: p
                .file_name()
                .map(|s| s.to_string_lossy().into_owned())
                .unwrap_or_else(|| p.to_string_lossy().into_owned()),
        })
        .collect();

    let mut all_skipped: Vec<SkippedRequestFile> = Vec::new();
    let mut request_groups: Vec<GroupedFiles> = Vec::new();
    let mut script_groups: Vec<GroupedFiles> = Vec::new();

    for (i, root) in canon.iter().enumerate() {
        let raw_requests = list_request_paths(root)?;
        let (valid, skipped) = partition_valid_request_paths(&raw_requests);
        for s in &skipped {
            debug!(
                path = %s.path.display(),
                err = %s.message,
                "skipped invalid request file"
            );
        }
        all_skipped.extend(skipped);

        let entries: Vec<FileEntry> = valid
            .into_iter()
            .map(|p| FileEntry {
                path: p.to_string_lossy().to_string(),
                name: p
                    .file_name()
                    .map(|s| s.to_string_lossy().into_owned())
                    .unwrap_or_default(),
            })
            .collect();

        let label = root_infos[i].label.clone();
        request_groups.push(GroupedFiles {
            root_index: i,
            root_label: label.clone(),
            entries,
        });

        let raw_scripts = list_rhai_paths(root)?;
        let script_entries: Vec<FileEntry> = raw_scripts
            .into_iter()
            .map(|p| FileEntry {
                path: p.to_string_lossy().to_string(),
                name: p
                    .file_name()
                    .map(|s| s.to_string_lossy().into_owned())
                    .unwrap_or_default(),
            })
            .collect();

        script_groups.push(GroupedFiles {
            root_index: i,
            root_label: label,
            entries: script_entries,
        });
    }

    Ok(WorkspaceSnapshot {
        roots: root_infos,
        requests: request_groups,
        scripts: script_groups,
        skipped_requests: all_skipped,
    })
}

pub fn dist_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("frontend/dist")
}
