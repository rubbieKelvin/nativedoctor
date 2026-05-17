use std::path::PathBuf;

pub fn default_new_project_candidate() -> Option<(String, PathBuf)> {
    let home = dirs::home_dir()?;

    let display_name = "Untitled Project";
    let location = home;

    Some((display_name.to_string(), location))
}
