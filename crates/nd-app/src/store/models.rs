use std::path::PathBuf;

pub struct RecentProject {
    name: String,
    path: PathBuf,
}

pub struct NativedoctorProjectFile {
    name: String,
    // folders: (id, name)
    folders: Vec<(String, String)>,
    // requests: file path, folder id
    requests: Vec<(PathBuf, Option<String>)>,
    // path to the md file that holds the root docs
    docs: Option<PathBuf>,
}
