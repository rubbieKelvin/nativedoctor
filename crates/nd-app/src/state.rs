//! Central application state.
//!
//! [`AppState`] is the root model that drives navigation and holds the
//! currently-open project. It is stored as a GPUI [`Entity`] and shared across
//! pages and components.

use nd_db::models::{self, RecentProject};
use std::path::PathBuf;
use std::sync::Arc;

/// The top-level view the user is currently seeing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum View {
    /// Landing page with recent projects plus create/open actions.
    Landing,
    /// Main workspace for an open project.
    Workspace,
}

/// Which tab is active in the workspace sidebar navigator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SidebarTab {
    /// Show the requests tree (folders plus requests).
    Requests,
    /// Show the Rhai tests list.
    Tests,
}

/// Indices for the Kyoshi-like bottom dock tab strip.
#[repr(usize)]
#[allow(dead_code)]
pub enum WorkspaceBottomTab {
    /// Application / request logs placeholder.
    Logs = 0,
    /// Scripted console placeholder.
    Console = 1,
    /// Structured test-results placeholder.
    Results = 2,
}

/// Everything the app needs to render for an open project.
#[derive(Clone)]
pub struct ActiveProject {
    /// The database row for this project.
    pub project: models::Project,
    /// Absolute path to the `.db` file on disk.
    pub db_path: PathBuf,
    /// Database store handle (`Arc` so it can be shared with async tasks).
    pub store: Arc<nd_db::store::Store>,
    /// Folders loaded from the DB (refreshed after mutations later).
    pub folders: Vec<models::Folder>,
    /// All requests in the project.
    pub requests: Vec<models::Request>,
    /// All test scripts.
    pub tests: Vec<models::Test>,
    /// All named environments.
    pub environments: Vec<models::Environment>,
    /// Which sidebar navigator tab is active.
    pub selected_tab: SidebarTab,
    /// The request ID currently loaded in the editor.
    pub selected_request_id: Option<String>,
    /// The test ID currently loaded in the test editor.
    pub selected_test_id: Option<String>,
    /// The environment whose variables are applied during execution.
    pub active_environment_id: Option<String>,
    /// Unsaved edits to the currently-selected request (if any).
    #[allow(dead_code)]
    pub dirty_request: Option<models::Request>,
    /// The most recent execution result for display in the response pane.
    pub last_execution_result: Option<ExecutionResultState>,
    /// Selected bottom dock tab (`Logs`, `Console`, `Results`).
    pub bottom_panel_tab: usize,
}

impl ActiveProject {
    /// Open the database file and hydrate all collections required by the navigator.
    ///
    /// If no `project` row exists yet an empty migrated database still creates a sensible default row.
    pub async fn open_and_load(db_path: PathBuf) -> anyhow::Result<Self> {
        let store = Arc::new(nd_db::store::Store::open(&db_path).await?);

        let mut projects = store.list_projects().await?;

        let project = if projects.is_empty() {
            let created = models::Project::new("Untitled Project");
            store.insert_project(&created).await?
        } else {
            // `list_projects` is ordered newest-first — treat that as authoritative.
            projects.swap_remove(0)
        };

        let pid = project.id.clone();

        let folders = store.list_folders(&pid).await?;
        let requests = store.list_requests(&pid, None).await?;
        let tests = store.list_tests(&pid).await?;
        let environments = store.list_environments(&pid).await?;

        Ok(Self {
            project,
            db_path,
            store,
            folders,
            requests,
            tests,
            environments,
            selected_tab: SidebarTab::Requests,
            selected_request_id: None,
            selected_test_id: None,
            active_environment_id: None,
            dirty_request: None,
            last_execution_result: None,
            bottom_panel_tab: WorkspaceBottomTab::Logs as usize,
        })
    }

    /// Persist a freshly created project row and hydrate empty collections without duplicating lookups.
    pub async fn bootstrap_new(name: impl Into<String>, db_path: PathBuf) -> anyhow::Result<Self> {
        let store = Arc::new(nd_db::store::Store::open(&db_path).await?);
        let project = models::Project::new(name);
        store.insert_project(&project).await?;
        let pid = project.id.clone();

        let folders = store.list_folders(&pid).await?;
        let requests = store.list_requests(&pid, None).await?;
        let tests = store.list_tests(&pid).await?;
        let environments = store.list_environments(&pid).await?;

        Ok(Self {
            project,
            db_path,
            store,
            folders,
            requests,
            tests,
            environments,
            selected_tab: SidebarTab::Requests,
            selected_request_id: None,
            selected_test_id: None,
            active_environment_id: None,
            dirty_request: None,
            last_execution_result: None,
            bottom_panel_tab: WorkspaceBottomTab::Logs as usize,
        })
    }

    /// Reload folders, requests, tests, environments and the canonical project row.
    ///
    /// Preserves ephemeral UI fields such as sidebar tab and selection identifiers when possible.
    pub async fn reload_from_store(&mut self) -> anyhow::Result<()> {
        let store = Arc::clone(&self.store);

        let project = match store.get_project(&self.project.id).await? {
            Some(p) => p,
            None => anyhow::bail!("Project row disappeared unexpectedly"),
        };

        let pid = project.id.clone();
        let folders = store.list_folders(&pid).await?;
        let requests = store.list_requests(&pid, None).await?;
        let tests = store.list_tests(&pid).await?;
        let environments = store.list_environments(&pid).await?;

        if let Some(id) = &self.selected_request_id {
            if !requests.iter().any(|r| r.id == *id) {
                self.selected_request_id = None;
            }
        }

        if let Some(id) = &self.selected_test_id {
            if !tests.iter().any(|t| t.id == *id) {
                self.selected_test_id = None;
            }
        }

        self.project = project;
        self.folders = folders;
        self.requests = requests;
        self.tests = tests;
        self.environments = environments;

        return Ok(());
    }
}

/// Lightweight representation of a completed HTTP call for the UI.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ExecutionResultState {
    pub method: String,
    pub url: String,
    pub status: u16,
    pub duration_ms: u64,
    pub response_headers: Vec<(String, String)>,
    pub response_body: Vec<u8>,
    pub response_size: usize,
    pub error_message: Option<String>,
}

/// Root application state.
pub struct AppState {
    /// Which coarse page is rendered.
    pub current_view: View,
    /// Recently opened projects surfaced on the landing page.
    pub recent_projects: Vec<RecentProject>,
    /// When [`Some`] a project DB is mounted and workspace chrome is usable.
    pub active_project: Option<ActiveProject>,
}

impl AppState {
    /// Create the initial shell state — lands on [`View::Landing`].
    pub fn new() -> Self {
        let recent_projects = Self::load_recent_projects().unwrap_or_else(|err| {
            tracing::warn!("Could not load recent projects: {err}");
            Vec::new()
        });

        return Self {
            current_view: View::Landing,
            recent_projects,
            active_project: None,
        };
    }

    /// Return to landing and drop the mounted database handle.
    pub fn navigate_to_landing(&mut self) -> () {
        self.active_project = None;
        self.current_view = View::Landing;

        return;
    }

    /// Swap in a freshly-loaded [`ActiveProject`] and flip to workspace.
    pub fn attach_open_project(&mut self, active: ActiveProject) -> () {
        self.touch_recent_project(&active.project.name, &active.db_path.to_string_lossy());
        self.active_project = Some(active);
        self.current_view = View::Workspace;

        return;
    }

    /// Persist the sidebar tab for the workspace navigator.
    pub fn set_sidebar_tab(&mut self, tab: SidebarTab) -> () {
        let Some(project) = self.active_project.as_mut() else {
            return;
        };

        project.selected_tab = tab;

        return;
    }

    /// Select a request row and implicitly clear conflicting test selection.
    pub fn select_request(&mut self, id: impl Into<Option<String>>) -> () {
        let Some(project) = self.active_project.as_mut() else {
            return;
        };

        let id = id.into();
        project.selected_request_id = id;
        project.selected_test_id = None;

        return;
    }

    /// Select a Rhai test row and clear request selection instead.
    pub fn select_test(&mut self, id: impl Into<Option<String>>) -> () {
        let Some(project) = self.active_project.as_mut() else {
            return;
        };

        let id = id.into();
        project.selected_test_id = id;
        project.selected_request_id = None;

        return;
    }

    /// Persist which bottom-panel tab strip is highlighted.
    pub fn set_workspace_bottom_tab(&mut self, index: usize) -> () {
        let Some(project) = self.active_project.as_mut() else {
            return;
        };

        project.bottom_panel_tab = index;

        return;
    }

    /// Persist the recent-projects manifest to disk.
    pub fn save_recent_projects(list: &[RecentProject]) -> Result<(), anyhow::Error> {
        let dir = dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("Cannot determine home directory"))?
            .join(".nativedoctor");

        std::fs::create_dir_all(&dir)?;

        let path = dir.join("recent.json");
        let json = serde_json::to_string_pretty(list)?;
        std::fs::write(&path, json)?;

        return Ok(());
    }

    /// Bump a project entry to the top of [`Self::recent_projects`].
    pub fn touch_recent_project(&mut self, name: &str, db_path: &str) -> () {
        self.recent_projects.retain(|p| p.db_path != db_path);

        self.recent_projects.insert(
            0,
            RecentProject {
                name: name.to_string(),
                db_path: db_path.to_string(),
                last_opened: chrono::Utc::now().to_rfc3339(),
            },
        );

        self.recent_projects.truncate(10);

        let _ignored = Self::save_recent_projects(&self.recent_projects);

        return;
    }

    /// Load recent projects saved under `~/.nativedoctor/recent.json`.
    fn load_recent_projects() -> Result<Vec<RecentProject>, anyhow::Error> {
        let path = dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("Cannot determine home directory"))?
            .join(".nativedoctor")
            .join("recent.json");

        if !path.exists() {
            return Ok(Vec::new());
        }

        let json = std::fs::read_to_string(&path)?;
        let list: Vec<RecentProject> = serde_json::from_str(&json)?;

        return Ok(list);
    }
}
