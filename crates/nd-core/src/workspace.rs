//! Ties a workspace's manifest, OpenAPI documents, and environment files together. This is the
//! main entry point `nd-cli` and `nd-desktop` use to open a workspace folder.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use nd_constants::WORKSPACE_MANIFEST_FILE;

use crate::env::RuntimeEnv;
use crate::error::{Error, Result};
use crate::model::{ApiDocument, Manifest, RequestRef, ResolvedRequest};

pub struct Workspace {
    pub manifest: Manifest,
    /// Loaded documents keyed by path relative to the workspace root (matches
    /// [`Manifest::openapi_docs`] entries and [`RequestRef::doc`]).
    pub docs: HashMap<PathBuf, ApiDocument>,
}

impl Workspace {
    /// Open a workspace that already has a `nativedoctor.yaml` manifest at its root.
    pub fn open(root: &Path) -> Result<Self> {
        let manifest_path = root.join(WORKSPACE_MANIFEST_FILE);
        let manifest = Manifest::load(&manifest_path)?;
        Self::from_manifest(manifest)
    }

    fn from_manifest(manifest: Manifest) -> Result<Self> {
        let root = manifest
            .root()
            .ok_or_else(|| Error::Plain("manifest has no root path".into()))?
            .to_path_buf();

        let mut docs = HashMap::new();
        for rel in &manifest.openapi_docs {
            let doc = ApiDocument::load(&root.join(rel))?;
            docs.insert(rel.clone(), doc);
        }

        Ok(Self { manifest, docs })
    }

    pub fn root(&self) -> &Path {
        // `from_manifest` only ever constructs with a manifest that has a root.
        self.manifest.root().expect("workspace manifest has a root")
    }

    pub fn doc(&self, rel: &Path) -> Option<&ApiDocument> {
        self.docs.get(rel)
    }

    pub fn doc_mut(&mut self, rel: &Path) -> Option<&mut ApiDocument> {
        self.docs.get_mut(rel)
    }

    pub fn resolve(&self, r: &RequestRef) -> Result<ResolvedRequest> {
        let doc = self.doc(&r.doc).ok_or_else(|| Error::OperationNotFound {
            doc: r.doc.clone(),
            path: r.path.clone(),
            method: r.method.as_str().to_string(),
        })?;
        doc.resolve_request(&r.path, r.method)
    }

    /// Build a [`RuntimeEnv`] from the manifest's `env_files`, plus a git-ignored
    /// `<file>.local` sibling for each one when it exists (e.g. `.env` + `.env.local`).
    pub fn load_env(&self) -> Result<RuntimeEnv> {
        let root = self.root();
        let env = RuntimeEnv::new();
        for rel in &self.manifest.env_files {
            let path = root.join(rel);
            if path.is_file() {
                env.merge_env_file(&path)?;
            }
            let local = local_sibling(&path);
            if local.is_file() {
                env.merge_env_file(&local)?;
            }
        }
        Ok(env)
    }
}

fn local_sibling(path: &Path) -> PathBuf {
    let mut name = path
        .file_name()
        .map(|n| n.to_os_string())
        .unwrap_or_default();
    name.push(".local");
    path.with_file_name(name)
}
