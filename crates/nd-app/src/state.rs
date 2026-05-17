//! Central application state.

use std::path::PathBuf;

use crate::store;

/// The top-level view the user is currently seeing.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum PageView {
    Landing,
    Workspace,
}

/// Which tab is active in the workspace sidebar navigator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SidebarTab {
    Requests,
    Tests,
}

/// Indices for the bottom dock tab strip.
#[repr(usize)]
#[allow(dead_code)]
pub enum WorkspaceBottomTab {
    Logs = 0,
    Console = 1,
    Results = 2,
}

// ── Runtime models loaded from the project directory ──────────────────────

#[derive(Debug, Clone)]
pub struct Project {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Request {
    pub id: String,
    pub name: String,
    pub method: String,
    pub url: String,
    pub summary: String,
    pub folder_id: Option<String>,
    pub headers: String,
    pub body_content: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Folder {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Test {
    pub id: String,
    pub name: String,
    pub script: String,
}

#[derive(Debug, Clone)]
pub struct Environment {
    pub id: String,
    pub name: String,
}

// ── ActiveProject ──────────────────────────────────────────────────────────

#[derive(Clone)]
pub struct ActiveProject {
    pub project: Project,
    pub db_path: PathBuf,
    pub folders: Vec<Folder>,
    pub requests: Vec<Request>,
    pub tests: Vec<Test>,
    pub environments: Vec<Environment>,
    pub selected_tab: SidebarTab,
    pub selected_request_id: Option<String>,
    pub selected_test_id: Option<String>,
    pub active_environment_id: Option<String>,
    pub last_execution_result: Option<ExecutionResultState>,
    pub bottom_panel_tab: usize,
}

impl ActiveProject {
    #[allow(dead_code)]
    pub async fn open(_file: PathBuf) -> anyhow::Result<Self> {
        todo!()
    }

    #[allow(dead_code)]
    pub async fn new(_name: impl Into<String>) -> anyhow::Result<Self> {
        todo!()
    }

    #[allow(dead_code)]
    pub async fn reload(&mut self) -> anyhow::Result<()> {
        todo!()
    }
}

// ── ExecutionResultState ───────────────────────────────────────────────────

#[derive(Debug, Clone)]
#[allow(dead_code)]
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

// ── AppState ───────────────────────────────────────────────────────────────

/// Root application state.
pub struct AppState {
    pub current_view: PageView,
    pub recent_projects: Vec<store::models::RecentProject>,
    pub active_project: Option<ActiveProject>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            current_view: PageView::Landing,
            recent_projects: Vec::new(),
            active_project: None,
        }
    }

    pub fn navigate_to_landing(&mut self) {
        self.current_view = PageView::Landing;
        self.active_project = None;
    }

    pub fn select_request(&mut self, id: Option<String>) {
        if let Some(ref mut project) = self.active_project {
            project.selected_request_id = id;
            project.selected_tab = SidebarTab::Requests;
        }
    }

    pub fn select_test(&mut self, id: Option<String>) {
        if let Some(ref mut project) = self.active_project {
            project.selected_test_id = id;
            project.selected_tab = SidebarTab::Tests;
        }
    }

    pub fn set_sidebar_tab(&mut self, tab: SidebarTab) {
        if let Some(ref mut project) = self.active_project {
            project.selected_tab = tab;
        }
    }

    pub fn set_workspace_bottom_tab(&mut self, index: usize) {
        if let Some(ref mut project) = self.active_project {
            project.bottom_panel_tab = index;
        }
    }
}
