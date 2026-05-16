//! Row types that map directly to SQLite tables. Each struct derives `Serialize` /
//! `Deserialize` so callers can easily convert to/from JSON and `FromRow` for sqlx
//! reading.

use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// ---------------------------------------------------------------------------
// Project
// ---------------------------------------------------------------------------

/// A top-level project (equivalent to a Postman "Workspace").
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Project {
    /// UUID v4 primary key.
    pub id: String,
    /// Human-readable name.
    pub name: String,
    /// Optional description.
    pub description: String,
    /// Base-level environment variables stored as a JSON object.
    pub base_env: String,
    /// ISO-8601 creation timestamp.
    pub created_at: String,
    /// ISO-8601 last-update timestamp.
    pub updated_at: String,
}

impl Project {
    /// Create a new project with the given `name`. All other fields are filled with
    /// sensible defaults.
    pub fn new(name: impl Into<String>) -> Self {
        let now = Utc::now().to_rfc3339();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.into(),
            description: String::new(),
            base_env: "{}".to_string(),
            created_at: now.clone(),
            updated_at: now,
        }
    }
}

// ---------------------------------------------------------------------------
// Folder
// ---------------------------------------------------------------------------

/// A folder for grouping requests within a project. Folders can be nested via
/// `parent_id`.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Folder {
    /// UUID v4 primary key.
    pub id: String,
    /// The project this folder belongs to.
    pub project_id: String,
    /// Optional parent folder for nesting; `None` means top-level.
    pub parent_id: Option<String>,
    /// Display name.
    pub name: String,
    /// Sort order within the parent (lower = first).
    pub sort_order: i32,
    pub created_at: String,
    pub updated_at: String,
}

impl Folder {
    /// Create a new folder within a project.
    pub fn new(project_id: impl Into<String>, name: impl Into<String>) -> Self {
        let now = Utc::now().to_rfc3339();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            project_id: project_id.into(),
            parent_id: None,
            name: name.into(),
            sort_order: 0,
            created_at: now.clone(),
            updated_at: now,
        }
    }
}

// ---------------------------------------------------------------------------
// Request (maps conceptually to `nd_core::model::request::RequestFile`)
// ---------------------------------------------------------------------------

/// A single HTTP request stored in the database. All fields mirror the on-disk
/// `RequestFile` / `HttpRequestSpec` types from `nd-core`.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Request {
    /// UUID v4 primary key.
    pub id: String,
    /// Owning project.
    pub project_id: String,
    /// Optional folder grouping.
    pub folder_id: Option<String>,
    /// Human-readable label.
    pub name: String,
    /// HTTP method (GET, POST, PUT, DELETE, …).
    pub method: String,
    /// URL template — may contain `${VAR}` placeholders.
    pub url: String,
    /// Short summary (OpenAPI-style metadata).
    pub summary: String,
    /// Longer description.
    pub description: String,
    /// JSON array of tags, e.g. `["users","auth"]`.
    pub tags: String,
    /// JSON object of header name → value.
    pub headers: String,
    /// JSON object of query parameter name → value.
    pub query_params: String,
    /// Body type discriminator: `json`, `text`, `xml`, `binary`,
    /// `form_data`, `x_www_form_urlencoded`, `graphql`, `other`, or `none`.
    pub body_type: Option<String>,
    /// The body payload — a JSON value for structured bodies, or a plain string.
    pub body_content: Option<String>,
    /// Request timeout in seconds (defaults to 30).
    pub timeout_secs: i64,
    /// Whether to follow HTTP redirects.
    pub follow_redirects: bool,
    /// Whether to verify TLS certificates.
    pub verify_tls: bool,
    /// Sort order within folder (lower = first).
    pub sort_order: i32,
    pub created_at: String,
    pub updated_at: String,
}

impl Request {
    /// Create a new request with sensible defaults.
    pub fn new(
        project_id: impl Into<String>,
        name: impl Into<String>,
        method: impl Into<String>,
        url: impl Into<String>,
    ) -> Self {
        let now = Utc::now().to_rfc3339();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            project_id: project_id.into(),
            folder_id: None,
            name: name.into(),
            method: method.into(),
            url: url.into(),
            summary: String::new(),
            description: String::new(),
            tags: "[]".to_string(),
            headers: "{}".to_string(),
            query_params: "{}".to_string(),
            body_type: None,
            body_content: None,
            timeout_secs: 30,
            follow_redirects: true,
            verify_tls: true,
            sort_order: 0,
            created_at: now.clone(),
            updated_at: now,
        }
    }
}

// ---------------------------------------------------------------------------
// Test (Rhai script)
// ---------------------------------------------------------------------------

/// A Rhai test script attached to a project.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Test {
    /// UUID v4 primary key.
    pub id: String,
    /// Owning project.
    pub project_id: String,
    /// Display name for the test.
    pub name: String,
    /// Raw Rhai source code.
    pub script: String,
    pub created_at: String,
    pub updated_at: String,
}

impl Test {
    /// Create a new test entry with an empty script body.
    pub fn new(project_id: impl Into<String>, name: impl Into<String>) -> Self {
        let now = Utc::now().to_rfc3339();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            project_id: project_id.into(),
            name: name.into(),
            script: String::new(),
            created_at: now.clone(),
            updated_at: now,
        }
    }
}

// ---------------------------------------------------------------------------
// Environment
// ---------------------------------------------------------------------------

/// A named set of variables (e.g. "dev", "staging", "prod").
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Environment {
    /// UUID v4 primary key.
    pub id: String,
    /// Owning project.
    pub project_id: String,
    /// Display name.
    pub name: String,
    /// JSON object of variable name → value pairs.
    pub variables: String,
    pub created_at: String,
    pub updated_at: String,
}

impl Environment {
    /// Create a new environment with empty variables.
    pub fn new(project_id: impl Into<String>, name: impl Into<String>) -> Self {
        let now = Utc::now().to_rfc3339();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            project_id: project_id.into(),
            name: name.into(),
            variables: "{}".to_string(),
            created_at: now.clone(),
            updated_at: now,
        }
    }
}

// ---------------------------------------------------------------------------
// ExecutionHistory
// ---------------------------------------------------------------------------

/// One row per executed request (used for the timeline, reports, and diffs).
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ExecutionHistory {
    /// UUID v4 primary key.
    pub id: String,
    /// Owning project.
    pub project_id: String,
    /// The request that was executed (nullable in case the request was deleted).
    pub request_id: Option<String>,
    /// The test script that triggered this execution (nullable for ad-hoc runs).
    pub test_id: Option<String>,
    /// Groups requests that were run together (batch / collection run).
    pub run_id: String,
    /// HTTP method used.
    pub method: String,
    /// Fully expanded URL (after template resolution).
    pub url: String,
    /// HTTP response status code, or `0` for network errors.
    pub status: i64,
    /// Wall-clock duration in milliseconds.
    pub duration_ms: i64,
    /// JSON-serialised request headers sent.
    pub request_headers: Option<String>,
    /// JSON-serialised response headers received.
    pub response_headers: Option<String>,
    /// Raw response body bytes. Large bodies may be truncated in the future.
    pub response_body: Option<Vec<u8>>,
    /// Size of the response body in bytes.
    pub response_size: i64,
    /// Error message when the request failed.
    pub error_message: Option<String>,
    /// ISO-8601 timestamp of the execution.
    pub created_at: String,
}

impl ExecutionHistory {
    /// Create a new execution-history entry.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        project_id: impl Into<String>,
        request_id: Option<String>,
        test_id: Option<String>,
        run_id: impl Into<String>,
        method: impl Into<String>,
        url: impl Into<String>,
        status: u16,
        duration_ms: u64,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            project_id: project_id.into(),
            request_id,
            test_id,
            run_id: run_id.into(),
            method: method.into(),
            url: url.into(),
            status: status as i64,
            duration_ms: duration_ms as i64,
            request_headers: None,
            response_headers: None,
            response_body: None,
            response_size: 0,
            error_message: None,
            created_at: Utc::now().to_rfc3339(),
        }
    }
}

// ---------------------------------------------------------------------------
// Recent project (lightweight, stored in ~/.nativedoctor/recent.json)
// ---------------------------------------------------------------------------

/// A recently-opened project entry kept in a small JSON file so the landing page
/// can list previous projects without scanning the filesystem.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentProject {
    /// Display name of the project.
    pub name: String,
    /// Absolute path to the `.db` file.
    pub db_path: String,
    /// ISO-8601 timestamp of when it was last opened.
    pub last_opened: String,
}
