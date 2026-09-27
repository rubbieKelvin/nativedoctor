//! Workspace manifest (`nativedoctor.yaml`): the sidebar tree, the OpenAPI documents that back it,
//! and where to find environment files. This is the one place tree position lives — OpenAPI
//! documents are grouped by path/tag on disk however is natural for the API, independent of how the
//! user wants to browse them in the desktop app.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::model::api::{ApiDocument, HttpMethod, RequestExample, RequestRef, SendBody};

/// One entry in the sidebar tree.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ManifestNode {
    Request(RequestRef),
    Folder {
        id: String,
        label: String,
        #[serde(default)]
        children: Vec<ManifestNode>,
    },
}

impl ManifestNode {
    pub fn folder(label: impl Into<String>) -> Self {
        ManifestNode::Folder {
            id: nanoid::nanoid!(),
            label: label.into(),
            children: Vec::new(),
        }
    }
}

/// The structure of a nativedoctor workspace's `nativedoctor.yaml`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub title: String,
    /// OpenAPI documents in this workspace, relative to the workspace root.
    #[serde(default)]
    pub openapi_docs: Vec<PathBuf>,
    /// Sidebar tree of requests/folders.
    #[serde(default)]
    pub tree: Vec<ManifestNode>,
    /// `.env`-style files providing default environment values (relative to the workspace root),
    /// merged in order (later files override earlier ones). Convention: a versioned `.env` for
    /// shared defaults, plus a git-ignored `.env.local` for per-developer secrets.
    #[serde(default)]
    pub env_files: Vec<PathBuf>,
    #[serde(skip)]
    pub _path: Option<PathBuf>,
}

impl Manifest {
    pub fn new(title: impl Into<String>) -> Self {
        Manifest {
            title: title.into(),
            openapi_docs: Vec::new(),
            tree: Vec::new(),
            env_files: Vec::new(),
            _path: None,
        }
    }

    pub fn load(path: &Path) -> Result<Self> {
        let text = std::fs::read_to_string(path)?;
        let mut manifest: Manifest =
            serde_yaml::from_str(&text).map_err(|e| Error::ParseYaml {
                path: path.to_path_buf(),
                source: e,
            })?;
        manifest._path = Some(path.to_path_buf());
        Ok(manifest)
    }

    /// Directory containing this manifest (the workspace root), if loaded from disk.
    pub fn root(&self) -> Option<&Path> {
        self._path.as_deref().and_then(Path::parent)
    }

    pub fn save(&self) -> Result<()> {
        let path = self
            ._path
            .clone()
            .ok_or_else(|| Error::Plain("manifest has no path set".into()))?;
        let content = serde_yaml::to_string(self).map_err(|e| {
            Error::Plain(format!("failed to serialize workspace manifest: {e}"))
        })?;
        Ok(std::fs::write(path, content)?)
    }

    pub fn add_request(&mut self, request: RequestRef) {
        self.tree.push(ManifestNode::Request(request));
    }

    /// Create a new workspace at `root/name`: manifest, one OpenAPI document with a starter
    /// request, and a `.env` with a placeholder `BASE_URL`.
    pub fn create_in_path(root: &Path, name: &str) -> Result<PathBuf> {
        if !root.is_dir() {
            return Err(Error::Plain("workspace root must be a directory".into()));
        }

        let project_dir = root.join(name);
        std::fs::create_dir_all(&project_dir)?;

        let openapi_path = project_dir.join("openapi.yaml");
        let mut doc = ApiDocument::create(&openapi_path, name, "${BASE_URL}")?;

        let example = RequestExample::default();
        let mut op = oas3::spec::Operation {
            summary: Some("Anything".to_string()),
            responses: Some(Default::default()),
            ..Default::default()
        };
        example.write_into(&mut op)?;
        doc.upsert_operation("/anything", HttpMethod::Get, op);

        let post_example = RequestExample {
            body: Some(SendBody::Json(serde_json::json!({"key": "value"}))),
            ..Default::default()
        };
        let mut post_op = oas3::spec::Operation {
            summary: Some("Anything (POST)".to_string()),
            responses: Some(Default::default()),
            ..Default::default()
        };
        post_example.write_into(&mut post_op)?;
        doc.upsert_operation("/anything", HttpMethod::Post, post_op);

        doc.save()?;

        let env_path = project_dir.join(".env");
        std::fs::write(&env_path, "BASE_URL=https://httpbin.org\n")?;

        let readme_path = project_dir.join("README.md");
        std::fs::write(
            &readme_path,
            format!("# {name}\n\nProject documentation for {name}.\n"),
        )?;

        let mut manifest = Manifest::new(name);
        manifest.openapi_docs.push(PathBuf::from("openapi.yaml"));
        manifest.env_files.push(PathBuf::from(".env"));
        manifest.add_request(RequestRef {
            doc: PathBuf::from("openapi.yaml"),
            path: "/anything".to_string(),
            method: HttpMethod::Get,
        });
        manifest.add_request(RequestRef {
            doc: PathBuf::from("openapi.yaml"),
            path: "/anything".to_string(),
            method: HttpMethod::Post,
        });
        manifest._path = Some(project_dir.join("nativedoctor.yaml"));
        manifest.save()?;

        Ok(project_dir)
    }
}
