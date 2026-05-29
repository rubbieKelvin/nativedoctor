use std::{
    fs,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use tracing::debug;

use crate::{
    error::{Error, Result},
    model::request::{HttpRequestSpec, RequestBody, RequestFile},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ResourceType {
    Request,
    Sequence,
    Environment,
}

impl ResourceType {
    pub fn from_id<S: AsRef<str>>(id: S) -> Option<Self> {
        let id = id.as_ref();

        return match id {
            n if n.starts_with("request:") => Some(Self::Request),
            n if n.starts_with("sequence:") => Some(Self::Sequence),
            n if n.starts_with("env:") => Some(Self::Environment),
            _ => None,
        };
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProjectResource {
    Request(PathBuf),
    Sequence(PathBuf),
    Environment(PathBuf),
    // folder id, label and children
    Folder(String, String, Vec<ProjectResource>),
}

impl ProjectResource {
    pub fn make_id(&self) -> String {
        return match self {
            Self::Request(p) => format!("request:{}", p.to_string_lossy()),
            Self::Environment(p) => format!("env:{}", p.to_string_lossy()),
            Self::Sequence(p) => format!("sequence:{}", p.to_string_lossy()),
            Self::Folder(id, ..) => format!("folder: {id}"),
        };
    }
}

/// the structure for a NativeDoctor project file
#[derive(Clone, Serialize, Deserialize)]
pub struct ProjectFile {
    pub title: String,
    pub requests: Vec<ProjectResource>,
    pub sequences: Vec<ProjectResource>,
    #[serde(skip)]
    pub _path: Option<PathBuf>,
}

impl ProjectFile {
    pub fn new<S: AsRef<str>>(title: S) -> Self {
        let title = title.as_ref();

        return ProjectFile {
            title: title.to_owned(),
            requests: vec![],
            sequences: vec![],
            _path: None,
        };
    }

    pub fn from_file(path: &Path) -> Result<Self> {
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();

        let text = std::fs::read_to_string(path)?;

        let mut file: Self = match ext.as_str() {
            "yaml" | "yml" => serde_yaml::from_str(&text).map_err(|e| Error::ParseYaml {
                path: path.to_path_buf(),
                source: e,
            })?,
            "json" => serde_json::from_str(&text).map_err(|e| Error::ParseJson {
                path: path.to_path_buf(),
                source: e,
            })?,
            _ => return Err(Error::UnsupportedFormat(path.to_path_buf())),
        };

        // Set meta
        file._path = Some(path.to_path_buf());

        debug!(
            path = %path.display(),
            format = %ext,
            name = ?&file.title,
            "loaded request file"
        );

        return Ok(file);
    }

    pub fn write(&self, path: PathBuf) -> Result<()> {
        let content = serde_yaml::to_string(self)
            .map_err(|e| Error::Plain(format!("Failed to serialize project file: {}", e)))?;

        return std::fs::write(&path, content).map_err(|e| {
            Error::Plain(format!(
                "Failed to write project file '{}': {}",
                path.display(),
                e
            ))
        });
    }

    pub fn create_in_path(root: PathBuf, name: String) -> Result<()> {
        if !root.is_dir() {
            return Err(Error::Plain("Root should be a directory".into()));
        }

        let project_dir = root.join(&name);

        fs::create_dir_all(&project_dir)
            .map_err(|e| Error::Plain(format!("Failed to create project directory: {}", e)))?;

        let requests_dir = project_dir.join("requests");
        fs::create_dir_all(&requests_dir)
            .map_err(|e| Error::Plain(format!("Failed to create requests directory: {}", e)))?;

        let request1 = RequestFile {
            name: Some("Test GET Request".into()),
            ..RequestFile::default()
        };
        let request1_path = requests_dir.join("request1.json");

        fs::write(
            &request1_path,
            serde_json::to_string_pretty(&request1)
                .map_err(|e| Error::Plain(format!("Failed to serialize request: {}", e)))?,
        )
        .map_err(|e| Error::Plain(format!("Failed to write request file: {}", e)))?;

        let request2 = RequestFile {
            name: Some("Test POST Request".into()),
            request: HttpRequestSpec {
                method: "POST".into(),
                url: "https://httpbin.org/post".into(),
                body: Some(RequestBody::Json(serde_json::json!({"key": "value"}))),
                ..RequestFile::default().request
            },
            ..RequestFile::default()
        };
        let request2_path = requests_dir.join("request2.json");
        fs::write(
            &request2_path,
            serde_json::to_string_pretty(&request2)
                .map_err(|e| Error::Plain(format!("Failed to serialize request: {}", e)))?,
        )
        .map_err(|e| Error::Plain(format!("Failed to write request file: {}", e)))?;

        let readme = format!("# {}\n\nProject documentation for {}.", name, name);
        let readme_path = project_dir.join("README.md");
        fs::write(&readme_path, readme.as_bytes())
            .map_err(|e| Error::Plain(format!("Failed to write README.md: {}", e)))?;

        let project_files = ProjectFile {
            title: name,
            requests: vec![
                ProjectResource::Request(PathBuf::from("requests/request1.json")),
                ProjectResource::Request(PathBuf::from("requests/request2.json")),
            ],
            sequences: vec![],
            _path: None,
        };

        fs::write(
            project_dir.join("nativedoctor.yaml"),
            serde_yaml::to_string(&project_files)
                .map_err(|e| Error::Plain(format!("Failed to serialize project file: {}", e)))?,
        )
        .map_err(|e| Error::Plain(format!("Failed to write .nativedoctor file: {}", e)))?;

        return Ok(());
    }
}
