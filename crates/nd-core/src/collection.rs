//! A collection is a folder with a `nativedoctor.yaml` at its root.

use std::fs;
use std::path::{Path, PathBuf};

use globset::{Glob, GlobSet, GlobSetBuilder};
use nd_constants::{DEFAULT_STATE, ENVIRONMENTS_DIR, LOCAL_DIR, MANIFEST_FILE, REQUEST_SUFFIX};
use walkdir::WalkDir;

use crate::error::{Error, Result};
use crate::manifest::{Environment, Manifest};

const SUPPORTED_VERSION: u32 = 1;

const STARTER_ENV: &str = "vars:\n  BASE_URL: https://httpbin.org\nsecrets: []\n";
const STARTER_REQUEST: &str = "name: Ping\nhttp:\n  method: GET\n  url: ${BASE_URL}/get\n";

#[derive(Debug, Clone)]
pub struct Collection {
    pub root: PathBuf,
    pub manifest: Manifest,
}

impl Collection {
    /// Walks up from `start` looking for a manifest.
    pub fn find(start: &Path) -> Result<Option<Collection>> {
        let start = start.canonicalize().map_err(|e| Error::io(start, e))?;
        for dir in start.ancestors() {
            if dir.join(MANIFEST_FILE).is_file() {
                return Ok(Some(Collection::open(dir)?));
            }
        }
        return Ok(None);
    }

    pub fn open(root: &Path) -> Result<Collection> {
        let path = root.join(MANIFEST_FILE);
        let text = fs::read_to_string(&path).map_err(|e| Error::io(&path, e))?;
        let manifest: Manifest = serde_yaml_ng::from_str(&text).map_err(|source| Error::Yaml {
            path: path.clone(),
            source,
        })?;
        if manifest.version > SUPPORTED_VERSION {
            return Err(Error::Invalid(format!(
                "{}: version {} needs a newer nativedoctor",
                path.display(),
                manifest.version
            )));
        }
        return Ok(Collection {
            root: root.to_path_buf(),
            manifest,
        });
    }

    /// Creates a starter collection in `dir`. Fails if one already exists there.
    pub fn init(dir: &Path, name: Option<&str>) -> Result<Collection> {
        fs::create_dir_all(dir).map_err(|e| Error::io(dir, e))?;
        let root = dir.canonicalize().map_err(|e| Error::io(dir, e))?;
        let manifest_path = root.join(MANIFEST_FILE);
        if manifest_path.exists() {
            return Err(Error::Invalid(format!(
                "{} already exists",
                manifest_path.display()
            )));
        }

        let fallback = root
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("collection");
        let manifest = Manifest {
            name: name.unwrap_or(fallback).to_string(),
            version: SUPPORTED_VERSION,
            default_env: Some("dev".to_string()),
            ..Manifest::default()
        };
        let text = serde_yaml_ng::to_string(&manifest).map_err(|source| Error::Yaml {
            path: manifest_path.clone(),
            source,
        })?;
        write_file(&manifest_path, &text)?;

        let env_dir = root.join(ENVIRONMENTS_DIR);
        fs::create_dir_all(&env_dir).map_err(|e| Error::io(&env_dir, e))?;
        write_if_absent(&env_dir.join("dev.yaml"), STARTER_ENV)?;
        write_if_absent(&root.join(format!("ping{REQUEST_SUFFIX}")), STARTER_REQUEST)?;

        let collection = Collection { root, manifest };
        crate::state::ensure_local_dir(&collection.local_dir())?;
        return Ok(collection);
    }

    pub fn local_dir(&self) -> PathBuf {
        self.root.join(LOCAL_DIR)
    }

    pub fn secrets_path(&self, env: &str) -> PathBuf {
        self.local_dir().join("secrets").join(format!("{env}.env"))
    }

    pub fn state_path(&self, env: Option<&str>) -> PathBuf {
        let name = env.unwrap_or(DEFAULT_STATE);
        return self.local_dir().join("state").join(format!("{name}.yaml"));
    }

    pub fn cookies_path(&self, env: Option<&str>) -> PathBuf {
        let name = env.unwrap_or(DEFAULT_STATE);
        return self
            .local_dir()
            .join("state")
            .join(format!("{name}.cookies.json"));
    }

    /// Names of the environments in `environments/`, sorted.
    pub fn environments(&self) -> Result<Vec<String>> {
        let dir = self.root.join(ENVIRONMENTS_DIR);
        if !dir.is_dir() {
            return Ok(Vec::new());
        }
        let mut names = Vec::new();
        for entry in fs::read_dir(&dir).map_err(|e| Error::io(&dir, e))? {
            let path = entry.map_err(|e| Error::io(&dir, e))?.path();
            if path.extension().is_some_and(|e| e == "yaml") {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    names.push(stem.to_string());
                }
            }
        }
        names.sort();
        return Ok(names);
    }

    fn environment_path(&self, name: &str) -> PathBuf {
        return self
            .root
            .join(ENVIRONMENTS_DIR)
            .join(format!("{name}.yaml"));
    }

    pub fn load_environment(&self, name: &str) -> Result<Environment> {
        let path = self.environment_path(name);
        if !valid_env_name(name) || !path.is_file() {
            return Err(Error::UnknownEnv {
                name: name.to_string(),
                available: self.environments()?.join(", "),
            });
        }
        let text = fs::read_to_string(&path).map_err(|e| Error::io(&path, e))?;
        return serde_yaml_ng::from_str(&text).map_err(|source| Error::Yaml { path, source });
    }

    /// Saves `nativedoctor.yaml`, keeping its comments.
    pub fn save_manifest(&self) -> Result<()> {
        return crate::patch::save_model(&self.root.join(MANIFEST_FILE), &self.manifest);
    }

    /// Saves an environment file, keeping its comments.
    pub fn save_environment(&self, name: &str, environment: &Environment) -> Result<()> {
        check_env_name(name)?;
        return crate::patch::save_model(&self.environment_path(name), environment);
    }

    /// Creates an empty environment. Fails if it already exists.
    pub fn create_environment(&self, name: &str) -> Result<()> {
        check_env_name(name)?;
        let path = self.environment_path(name);
        if path.exists() {
            return Err(Error::Invalid(format!("environment `{name}` already exists")));
        }
        return crate::patch::rewrite_model(&path, &Environment::default());
    }

    pub fn delete_environment(&self, name: &str) -> Result<()> {
        check_env_name(name)?;
        let path = self.environment_path(name);
        return fs::remove_file(&path).map_err(|e| Error::io(&path, e));
    }

    /// The collection's folders and requests, for browsing. Empty folders are included.
    pub fn tree(&self) -> Result<Vec<TreeNode>> {
        let exclude = build_excludes(&self.manifest.exclude)?;
        return Ok(self.walk_tree(&self.root, &exclude)?.0);
    }

    /// Returns the folder's contents, and whether anything in it was excluded.
    fn walk_tree(&self, dir: &Path, exclude: &GlobSet) -> Result<(Vec<TreeNode>, bool)> {
        let mut entries: Vec<fs::DirEntry> = fs::read_dir(dir)
            .map_err(|e| Error::io(dir, e))?
            .collect::<std::io::Result<_>>()
            .map_err(|e| Error::io(dir, e))?;
        entries.sort_by_key(fs::DirEntry::file_name);

        let mut nodes = Vec::new();
        let mut any_excluded = false;
        for entry in entries {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();
            let relative = path.strip_prefix(&self.root).unwrap_or(&path);
            let kind = entry.file_type().map_err(|e| Error::io(&path, e))?;
            if name.starts_with('.') || relative == Path::new(ENVIRONMENTS_DIR) {
                continue;
            }
            if exclude.is_match(relative) {
                any_excluded = true;
                continue;
            }
            if kind.is_dir() {
                let (children, excluded) = self.walk_tree(&path, exclude)?;
                // A folder emptied by excludes is excluded too. One that's really empty isn't.
                if children.is_empty() && excluded {
                    any_excluded = true;
                    continue;
                }
                nodes.push(TreeNode::Folder { name, path, children });
            } else if kind.is_file() && name.ends_with(REQUEST_SUFFIX) {
                nodes.push(TreeNode::Request { path });
            }
        }
        return Ok((nodes, any_excluded));
    }

    /// All request files in the collection, sorted by path.
    pub fn requests(&self) -> Result<Vec<PathBuf>> {
        return self.requests_under(&self.root);
    }

    /// Request files under `dir`, minus anything matching `exclude`.
    pub fn requests_under(&self, dir: &Path) -> Result<Vec<PathBuf>> {
        let exclude = build_excludes(&self.manifest.exclude)?;
        let files = discover(dir)?;
        return Ok(files
            .into_iter()
            .filter(|path| {
                let relative = path.strip_prefix(&self.root).unwrap_or(path);
                !exclude.is_match(relative)
            })
            .collect());
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TreeNode {
    Folder {
        name: String,
        path: PathBuf,
        children: Vec<TreeNode>,
    },
    Request {
        path: PathBuf,
    },
}

fn valid_env_name(name: &str) -> bool {
    return !name.is_empty()
        && !name.starts_with('.')
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'));
}

fn check_env_name(name: &str) -> Result<()> {
    if valid_env_name(name) {
        return Ok(());
    }
    return Err(Error::Invalid(format!(
        "`{name}` isn't a valid environment name; use letters, numbers, `-`, `_` and `.`"
    )));
}

/// Finds `*.request.yaml` under `dir`, skipping hidden folders.
pub fn discover(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    let walker = WalkDir::new(dir).sort_by_file_name().into_iter();
    let visible = walker.filter_entry(|entry| {
        let hidden = entry.file_name().to_string_lossy().starts_with('.');
        entry.depth() == 0 || !hidden
    });
    for entry in visible {
        let entry = entry.map_err(|e| Error::Invalid(format!("{}: {e}", dir.display())))?;
        let is_request = entry
            .file_name()
            .to_string_lossy()
            .ends_with(REQUEST_SUFFIX);
        if entry.file_type().is_file() && is_request {
            files.push(entry.into_path());
        }
    }
    return Ok(files);
}

fn build_excludes(patterns: &[String]) -> Result<GlobSet> {
    let mut builder = GlobSetBuilder::new();
    for pattern in patterns {
        let glob = Glob::new(pattern)
            .map_err(|e| Error::Invalid(format!("exclude pattern `{pattern}`: {e}")))?;
        builder.add(glob);
    }
    return builder
        .build()
        .map_err(|e| Error::Invalid(format!("exclude patterns: {e}")));
}

fn write_file(path: &Path, text: &str) -> Result<()> {
    fs::write(path, text).map_err(|e| Error::io(path, e))
}

fn write_if_absent(path: &Path, text: &str) -> Result<()> {
    if path.exists() {
        return Ok(());
    }
    return write_file(path, text);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn touch(path: &Path) {
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, "").unwrap();
    }

    #[test]
    fn init_creates_a_usable_collection() {
        let dir = tempfile::tempdir().unwrap();
        let collection = Collection::init(dir.path(), Some("Demo")).unwrap();
        assert_eq!(collection.manifest.name, "Demo");
        assert_eq!(collection.environments().unwrap(), vec!["dev"]);
        assert_eq!(collection.requests().unwrap().len(), 1);
        assert!(Collection::init(dir.path(), None).is_err());
    }

    #[test]
    fn find_walks_up_from_nested_folder() {
        let dir = tempfile::tempdir().unwrap();
        Collection::init(dir.path(), None).unwrap();
        let nested = dir.path().join("a/b");
        fs::create_dir_all(&nested).unwrap();
        let found = Collection::find(&nested).unwrap().unwrap();
        assert_eq!(found.root, dir.path().canonicalize().unwrap());
    }

    #[test]
    fn discovery_honours_excludes_and_hidden_folders() {
        let dir = tempfile::tempdir().unwrap();
        let mut collection = Collection::init(dir.path(), None).unwrap();
        touch(&dir.path().join("users/get.request.yaml"));
        touch(&dir.path().join("drafts/wip.request.yaml"));
        touch(&dir.path().join(".nativedoctor/x.request.yaml"));
        touch(&dir.path().join("users/notes.yaml"));
        collection.manifest.exclude = vec!["drafts/**".to_string()];

        let names: Vec<String> = collection
            .requests()
            .unwrap()
            .iter()
            .map(|p| {
                p.strip_prefix(&collection.root)
                    .unwrap()
                    .display()
                    .to_string()
            })
            .collect();
        assert_eq!(names, vec!["ping.request.yaml", "users/get.request.yaml"]);
    }

    #[test]
    fn tree_shows_empty_folders_but_not_excluded_ones() {
        let dir = tempfile::tempdir().unwrap();
        let mut collection = Collection::init(dir.path(), None).unwrap();
        touch(&dir.path().join("users/get.request.yaml"));
        touch(&dir.path().join("users/notes.txt"));
        touch(&dir.path().join("drafts/wip.request.yaml"));
        touch(&dir.path().join(".hidden/x.request.yaml"));
        fs::create_dir_all(dir.path().join("empty")).unwrap();
        collection.manifest.exclude = vec!["drafts/**".to_string()];

        let tree = collection.tree().unwrap();
        let names: Vec<String> = tree
            .iter()
            .map(|node| match node {
                TreeNode::Folder { name, children, .. } => format!("{name}/{}", children.len()),
                TreeNode::Request { path } => {
                    path.file_name().unwrap().to_string_lossy().to_string()
                }
            })
            .collect();
        assert_eq!(names, vec!["empty/0", "ping.request.yaml", "users/1"]);
    }

    #[test]
    fn saving_keeps_comments_in_manifest_and_environments() {
        let dir = tempfile::tempdir().unwrap();
        let mut collection = Collection::init(dir.path(), Some("Demo")).unwrap();
        let manifest = dir.path().join(MANIFEST_FILE);
        let text = fs::read_to_string(&manifest).unwrap();
        fs::write(&manifest, format!("# mine\n{text}")).unwrap();
        collection.manifest.name = "Renamed".to_string();
        collection.save_manifest().unwrap();
        let saved = fs::read_to_string(&manifest).unwrap();
        assert!(saved.starts_with("# mine\n") && saved.contains("Renamed"), "{saved}");

        collection.create_environment("prod").unwrap();
        assert!(collection.create_environment("prod").is_err());
        assert!(collection.create_environment("../evil").is_err());
        let mut env = collection.load_environment("prod").unwrap();
        env.secrets.push("TOKEN".to_string());
        collection.save_environment("prod", &env).unwrap();
        assert_eq!(collection.load_environment("prod").unwrap().secrets, vec!["TOKEN"]);
        collection.delete_environment("prod").unwrap();
        assert_eq!(collection.environments().unwrap(), vec!["dev"]);
    }

    #[test]
    fn environment_names_cant_escape_the_folder() {
        let dir = tempfile::tempdir().unwrap();
        let collection = Collection::init(dir.path(), None).unwrap();
        fs::write(dir.path().join("evil.yaml"), "vars: {}\n").unwrap();
        assert!(matches!(
            collection.load_environment("../evil"),
            Err(Error::UnknownEnv { .. })
        ));
        assert!(collection.load_environment("dev").is_ok());
    }
}
