//! Project creation / opening dialogs.
//!
//! These functions prompt the user for a project name and database path.
//! In a full GPUI app we would render modal dialogs; for the MVP we use
//! simple programmatic logic that can be replaced with real dialogs later.

/// Show a "Create New Project" dialog.
///
/// Returns `Some((project_name, db_path))` when the user confirms,
/// or `None` when cancelled.
pub async fn show_create_dialog(_cx: &mut gpui::AsyncWindowContext) -> Option<(String, String)> {
    // MVP: generate a default project name and path under ~/.nativedoctor/.
    // In a real implementation, this would show a GPUI modal with a text
    // input and a native file-save dialog.
    let home = dirs::home_dir()?;
    let nd_dir = home.join(".nativedoctor");
    std::fs::create_dir_all(&nd_dir).ok()?;

    let name = "Untitled Project";
    let sanitised = name.to_lowercase().replace(' ', "_");
    let db_path = nd_dir.join(format!("{sanitised}.db"));

    // If the file already exists, append a number.
    let db_path = unique_path(db_path);

    tracing::info!("Creating new project '{}' at {:?}", name, db_path);

    Some((name.to_string(), db_path.to_string_lossy().to_string()))
}

/// Show an "Open Existing Project" dialog.
///
/// Returns `Some((project_name, db_path))` when the user selects a file,
/// or `None` when cancelled.
pub async fn show_open_dialog(_cx: &mut gpui::AsyncWindowContext) -> Option<(String, String)> {
    // MVP: for now, this is a no-op placeholder. A real implementation
    // would use a native file-picker (e.g. `rfd` or GPUI's file dialog).
    // Until then, users can open recent projects from the landing page
    // list.
    tracing::info!("Open project dialog requested (not yet implemented)");

    None
}

/// Return a path that doesn't collide with an existing file by appending a
/// suffix like `_1`, `_2`, etc.
fn unique_path(path: std::path::PathBuf) -> std::path::PathBuf {
    if !path.exists() {
        return path;
    }

    let stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("project");
    let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("db");
    let parent = path.parent().unwrap_or(std::path::Path::new("."));

    for n in 1u32.. {
        let candidate = parent.join(format!("{stem}_{n}.{ext}"));
        if !candidate.exists() {
            return candidate;
        }
    }

    // Fallback (shouldn't happen).
    path
}
