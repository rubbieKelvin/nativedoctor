//! Project creation shortcuts that lean on deterministic `~/.nativedoctor` paths for now.

use std::path::PathBuf;

/// Build a deterministic project name/path pair usable for onboarding flows.
///
/// The database file is synthesised underneath `/.nativedoctor` mirroring CLI defaults until a GPUI-first dialog ships.
pub fn default_new_project_candidate() -> Option<(String, PathBuf)> {
    let home = dirs::home_dir()?;
    let nd_dir = home.join(".nativedoctor");

    std::fs::create_dir_all(&nd_dir).ok()?;

    let display_name = "Untitled Project";
    let sanitized = display_name.to_lowercase().replace(' ', "_");
    let candidate = nd_dir.join(format!("{sanitized}.db"));

    return Some((display_name.to_string(), unique_path(candidate)));
}

/// Produce a sibling filename if `path` already exists on disk.
fn unique_path(path: PathBuf) -> PathBuf {
    if !path.exists() {
        return path;
    }

    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("project");
    let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("db");
    let parent = path.parent().unwrap_or(std::path::Path::new("."));

    for suffix in 1u32.. {
        let contender = parent.join(format!("{stem}_{suffix}.{ext}"));
        if !contender.exists() {
            return contender;
        }
    }

    return path;
}
