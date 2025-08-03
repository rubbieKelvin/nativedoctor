use std::path::PathBuf;

/// The user may open the application with `nativedoctor filename.nativedoctor`, ie. opening a project
/// Or they might want to open a single request file.
pub enum AppSessionType {
    Project(PathBuf),
    SingleRequestFile(PathBuf),
}