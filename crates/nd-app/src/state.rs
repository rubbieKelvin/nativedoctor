//! Central application state.
//!
//! [`AppState`] is the root model that drives navigation and holds the
//! currently-open project. It is stored as a `gpui::Model<AppState>` and
//! shared across pages and components.

use nd_db::models::{self, RecentProject};
use std::path::PathBuf;
use std::sync::Arc;

/// The top-level view the user is currently seeing.
#[derive(Debug, Clone, PartialEq)]
pub enum View {
    /// Landing page with recent projects + create/open actions.
    Landing,
    /// Main workspace for an open project.
    Workspace,
}

/// Which tab is active in the workspace sidebar.
#[derive(Debug, Clone, PartialEq)]
pub enum SidebarTab {
    /// Show the requests tree (folders + requests).
    Requests,
    /// Show the tests list.
    Tests,
}

/// Everything the app needs to render for an open project.
pub struct ActiveProject {
    /// The database row for this project.
    pub project: models::Project,
    /// Absolute path to the `.db` file on disk.
    pub db_path: PathBuf,
    /// Database store handle (Arc so it can be shared with async tasks).
    pub store: Arc<nd_db::store::Store>,
    /// Folders loaded from the DB (refreshed on mutations).
    pub folders: Vec<models::Folder>,
    /// All requests in the project.
    pub requests: Vec<models::Request>,
    /// All test scripts.
    pub tests: Vec<models::Test>,
    /// All named environments.
    pub environments: Vec<models::Environment>,
    /// Which sidebar tab is active.
    pub selected_tab: SidebarTab,
    /// The request ID currently loaded in the editor.
    pub selected_request_id: Option<String>,
    /// The test ID currently loaded in the test editor.
    pub selected_test_id: Option<String>,
    /// The environment whose variables are applied during execution.
    pub active_environment_id: Option<String>,
    /// Unsaved edits to the currently-selected request (if any).
    pub dirty_request: Option<models::Request>,
    /// The most recent execution result for display in the response pane.
    pub last_execution_result: Option<ExecutionResultState>,
}

/// Lightweight representation of a completed HTTP call for the UI.
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
    /// Which view we are currently showing.
    pub current_view: View,
    /// List of recently-opened projects (loaded from disk).
    pub recent_projects: Vec<RecentProject>,
    /// When `Some`, a project is open and the workspace is visible.
    pub active_project: Option<ActiveProject>,
}

impl AppState {
    /// Create the initial state — always starts on the landing page.
    pub fn new() -> Self {
        let recent = Self::load_recent_projects().unwrap_or_default();

        Self {
            current_view: View::Landing,
            recent_projects: recent,
            active_project: None,
        }
    }

    /// Persist the recent-projects list to `~/.nativedoctor/recent.json`.
    pub fn save_recent_projects(list: &[RecentProject]) -> Result<(), anyhow::Error> {
        let dir = dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("Cannot determine home directory"))?
            .join(".nativedoctor");

        std::fs::create_dir_all(&dir)?;

        let path = dir.join("recent.json");
        let json = serde_json::to_string_pretty(list)?;
        std::fs::write(&path, json)?;

        Ok(())
    }

    /// Load the recent-projects list from disk.
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

        Ok(list)
    }

    /// Add (or move to front) a project in the recent list, then persist.
    pub fn touch_recent_project(&mut self, name: &str, db_path: &str) {
        // Remove any existing entry for this path.
        self.recent_projects.retain(|p| p.db_path != db_path);

        // Insert at the front.
        self.recent_projects.insert(
            0,
            RecentProject {
                name: name.to_string(),
                db_path: db_path.to_string(),
                last_opened: chrono::Utc::now().to_rfc3339(),
            },
        );

        // Cap at 10 entries.
        self.recent_projects.truncate(10);

        let _ = Self::save_recent_projects(&self.recent_projects);
    }
}
