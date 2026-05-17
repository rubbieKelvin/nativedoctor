//! Central application state.
use std::path::PathBuf;
// use std::sync::Arc;

use crate::store;

/// The top-level view the user is currently seeing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PageView {
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
    // /// The database row for this project.
    // pub project: models::Project,
    // pub store: Arc<store::Store>,
    // pub folders: Vec<models::Folder>,
    // /// All requests in the project.
    // pub requests: Vec<models::Request>,
    // /// All test scripts.
    // pub tests: Vec<models::Test>,
    // /// All named environments.
    // pub environments: Vec<models::Environment>,
    // /// Which sidebar navigator tab is active.
    // pub selected_tab: SidebarTab,
    // /// The request ID currently loaded in the editor.
    // pub selected_request_id: Option<String>,
    // /// The test ID currently loaded in the test editor.
    // pub selected_test_id: Option<String>,
    // /// The environment whose variables are applied during execution.
    // pub active_environment_id: Option<String>,
    // /// Unsaved edits to the currently-selected request (if any).
    // #[allow(dead_code)]
    // pub dirty_request: Option<models::Request>,
    // /// The most recent execution result for display in the response pane.
    // pub last_execution_result: Option<ExecutionResultState>,
    // /// Selected bottom dock tab (`Logs`, `Console`, `Results`).
    // pub bottom_panel_tab: usize,
}

impl ActiveProject {
    /// Open the database file and hydrate all collections required by the navigator.
    ///
    /// If no `project` row exists yet an empty migrated database still creates a sensible default row.
    pub async fn open(file: PathBuf) -> anyhow::Result<Self> {
        todo!()
    }

    /// Persist a freshly created project row and hydrate empty collections without duplicating lookups.
    pub async fn new(name: impl Into<String>) -> anyhow::Result<Self> {
        todo!()
    }

    /// Reload folders, requests, tests, environments and the canonical project row.
    ///
    /// Preserves ephemeral UI fields such as sidebar tab and selection identifiers when possible.
    pub async fn reload(&mut self) -> anyhow::Result<()> {
        todo!()
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
    pub current_view: PageView,
    /// Recently opened projects surfaced on the landing page.
    pub recent_projects: Vec<store::models::RecentProject>,
    /// When [`Some`] a project DB is mounted and workspace chrome is usable.
    pub active_project: Option<ActiveProject>,
}

impl AppState {
    /// Create the initial shell state — lands on [`PageView::Landing`].
    pub fn new() -> Self {
        // let recent_projects = Self::load_recent_projects().unwrap_or_else(|err| {
        //     tracing::warn!("Could not load recent projects: {err}");
        //     Vec::new()
        // });

        // return Self {
        //     current_view: PageView::Landing,
        //     recent_projects,
        //     active_project: None,
        // };
        todo!()
    }
}
