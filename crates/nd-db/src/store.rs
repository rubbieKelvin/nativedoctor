//! Central data-access layer. A [`Store`] wraps a single SQLite connection
//! (one per project `.db` file) and exposes typed CRUD methods for every entity.
//!
//! # Usage
//! ```ignore
//! let store = Store::open("/Users/me/.nativedoctor/my_project.db").await?;
//! let project = store.insert_project(Project::new("My API")).await?;
//! let req = store.insert_request(Request::new(&project.id, "Get users", "GET", "https://...")).await?;
//! ```

use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::SqlitePool;
use std::path::Path;
use std::str::FromStr;

use crate::models::*;

/// Manages all database access for a single project.
///
/// Internally holds a `sqlx::SqlitePool` (size 1 — SQLite is single-writer).
pub struct Store {
    pool: SqlitePool,
}

impl Store {
    // -----------------------------------------------------------------------
    // Lifecycle
    // -----------------------------------------------------------------------

    /// Open (or create) the SQLite database at `db_path` and run migrations.
    ///
    /// The parent directory must already exist. Foreign keys and WAL journal
    /// mode are enabled automatically.
    pub async fn open(db_path: &Path) -> Result<Self, anyhow::Error> {
        let path_str = db_path.to_string_lossy().to_string();

        let opts = SqliteConnectOptions::from_str(&format!("sqlite://{path_str}"))?
            .create_if_missing(true)
            .foreign_keys(true)
            .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal);

        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect_with(opts)
            .await?;

        let store = Self { pool };
        store.run_migrations().await?;

        Ok(store)
    }

    /// Open an in-memory database (useful for tests).
    #[cfg(test)]
    pub async fn open_in_memory() -> Result<Self, anyhow::Error> {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await?;

        let store = Self { pool };
        store.run_migrations().await?;

        Ok(store)
    }

    async fn run_migrations(&self) -> Result<(), anyhow::Error> {
        for stmt in crate::migrate::migration_statements() {
            sqlx::query(stmt).execute(&self.pool).await?;
        }
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Project
    // -----------------------------------------------------------------------

    /// Insert a new project row. Returns the same `Project` with its `id` set.
    pub async fn insert_project(&self, project: &Project) -> Result<Project, anyhow::Error> {
        let mut p = project.clone();
        p.updated_at = chrono::Utc::now().to_rfc3339();

        sqlx::query(
            "INSERT INTO project (id, name, description, base_env, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        )
        .bind(&p.id)
        .bind(&p.name)
        .bind(&p.description)
        .bind(&p.base_env)
        .bind(&p.created_at)
        .bind(&p.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(p)
    }

    /// Fetch a project by its UUID string.
    pub async fn get_project(&self, id: &str) -> Result<Option<Project>, anyhow::Error> {
        let project = sqlx::query_as::<_, Project>("SELECT * FROM project WHERE id = ?1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(project)
    }

    /// Update mutable fields of an existing project.
    pub async fn update_project(&self, project: &Project) -> Result<(), anyhow::Error> {
        let now = chrono::Utc::now().to_rfc3339();
        sqlx::query(
            "UPDATE project SET name = ?1, description = ?2, base_env = ?3, updated_at = ?4 WHERE id = ?5",
        )
        .bind(&project.name)
        .bind(&project.description)
        .bind(&project.base_env)
        .bind(&now)
        .bind(&project.id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Delete a project and all related rows (cascades).
    pub async fn delete_project(&self, id: &str) -> Result<(), anyhow::Error> {
        sqlx::query("DELETE FROM project WHERE id = ?1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Return every project in the database (typically just one per file, but
    /// the schema allows multiple).
    pub async fn list_projects(&self) -> Result<Vec<Project>, anyhow::Error> {
        let projects =
            sqlx::query_as::<_, Project>("SELECT * FROM project ORDER BY updated_at DESC")
                .fetch_all(&self.pool)
                .await?;

        Ok(projects)
    }

    // -----------------------------------------------------------------------
    // Folder
    // -----------------------------------------------------------------------

    /// Insert a new folder.
    pub async fn insert_folder(&self, folder: &Folder) -> Result<Folder, anyhow::Error> {
        let mut f = folder.clone();
        f.updated_at = chrono::Utc::now().to_rfc3339();

        sqlx::query(
            "INSERT INTO folder (id, project_id, parent_id, name, sort_order, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        )
        .bind(&f.id)
        .bind(&f.project_id)
        .bind(&f.parent_id)
        .bind(&f.name)
        .bind(f.sort_order)
        .bind(&f.created_at)
        .bind(&f.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(f)
    }

    /// Fetch a single folder by id.
    pub async fn get_folder(&self, id: &str) -> Result<Option<Folder>, anyhow::Error> {
        let folder = sqlx::query_as::<_, Folder>("SELECT * FROM folder WHERE id = ?1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(folder)
    }

    /// Update folder fields.
    pub async fn update_folder(&self, folder: &Folder) -> Result<(), anyhow::Error> {
        let now = chrono::Utc::now().to_rfc3339();
        sqlx::query(
            "UPDATE folder SET parent_id = ?1, name = ?2, sort_order = ?3, updated_at = ?4 WHERE id = ?5",
        )
        .bind(&folder.parent_id)
        .bind(&folder.name)
        .bind(folder.sort_order)
        .bind(&now)
        .bind(&folder.id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Delete a folder (children cascade).
    pub async fn delete_folder(&self, id: &str) -> Result<(), anyhow::Error> {
        sqlx::query("DELETE FROM folder WHERE id = ?1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// List all folders for a project, ordered by `sort_order`.
    pub async fn list_folders(&self, project_id: &str) -> Result<Vec<Folder>, anyhow::Error> {
        let folders = sqlx::query_as::<_, Folder>(
            "SELECT * FROM folder WHERE project_id = ?1 ORDER BY sort_order ASC",
        )
        .bind(project_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(folders)
    }

    // -----------------------------------------------------------------------
    // Request
    // -----------------------------------------------------------------------

    /// Insert a new request. Returns the `Request` with DB-assigned defaults.
    pub async fn insert_request(&self, request: &Request) -> Result<Request, anyhow::Error> {
        let mut r = request.clone();
        r.updated_at = chrono::Utc::now().to_rfc3339();

        sqlx::query(
            "INSERT INTO request (id, project_id, folder_id, name, method, url,
             summary, description, tags, headers, query_params,
             body_type, body_content,
             timeout_secs, follow_redirects, verify_tls, sort_order,
             created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19)",
        )
        .bind(&r.id)
        .bind(&r.project_id)
        .bind(&r.folder_id)
        .bind(&r.name)
        .bind(&r.method)
        .bind(&r.url)
        .bind(&r.summary)
        .bind(&r.description)
        .bind(&r.tags)
        .bind(&r.headers)
        .bind(&r.query_params)
        .bind(&r.body_type)
        .bind(&r.body_content)
        .bind(r.timeout_secs)
        .bind(r.follow_redirects)
        .bind(r.verify_tls)
        .bind(r.sort_order)
        .bind(&r.created_at)
        .bind(&r.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(r)
    }

    /// Fetch a single request.
    pub async fn get_request(&self, id: &str) -> Result<Option<Request>, anyhow::Error> {
        let request = sqlx::query_as::<_, Request>("SELECT * FROM request WHERE id = ?1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(request)
    }

    /// Update a request's mutable fields.
    pub async fn update_request(&self, request: &Request) -> Result<(), anyhow::Error> {
        let now = chrono::Utc::now().to_rfc3339();
        sqlx::query(
            "UPDATE request SET
                folder_id = ?1, name = ?2, method = ?3, url = ?4,
                summary = ?5, description = ?6, tags = ?7,
                headers = ?8, query_params = ?9,
                body_type = ?10, body_content = ?11,
                timeout_secs = ?12, follow_redirects = ?13, verify_tls = ?14,
                sort_order = ?15, updated_at = ?16
             WHERE id = ?17",
        )
        .bind(&request.folder_id)
        .bind(&request.name)
        .bind(&request.method)
        .bind(&request.url)
        .bind(&request.summary)
        .bind(&request.description)
        .bind(&request.tags)
        .bind(&request.headers)
        .bind(&request.query_params)
        .bind(&request.body_type)
        .bind(&request.body_content)
        .bind(request.timeout_secs)
        .bind(request.follow_redirects)
        .bind(request.verify_tls)
        .bind(request.sort_order)
        .bind(&now)
        .bind(&request.id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Delete a request.
    pub async fn delete_request(&self, id: &str) -> Result<(), anyhow::Error> {
        sqlx::query("DELETE FROM request WHERE id = ?1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// List all requests for a project, optionally filtered by folder.
    pub async fn list_requests(
        &self,
        project_id: &str,
        folder_id: Option<&str>,
    ) -> Result<Vec<Request>, anyhow::Error> {
        if let Some(fid) = folder_id {
            let requests = sqlx::query_as::<_, Request>(
                "SELECT * FROM request WHERE project_id = ?1 AND folder_id = ?2 ORDER BY sort_order ASC",
            )
            .bind(project_id)
            .bind(fid)
            .fetch_all(&self.pool)
            .await?;

            Ok(requests)
        } else {
            let requests = sqlx::query_as::<_, Request>(
                "SELECT * FROM request WHERE project_id = ?1 ORDER BY sort_order ASC",
            )
            .bind(project_id)
            .fetch_all(&self.pool)
            .await?;

            Ok(requests)
        }
    }

    // -----------------------------------------------------------------------
    // Test
    // -----------------------------------------------------------------------

    /// Insert a new test.
    pub async fn insert_test(&self, test: &Test) -> Result<Test, anyhow::Error> {
        let mut t = test.clone();
        t.updated_at = chrono::Utc::now().to_rfc3339();

        sqlx::query(
            "INSERT INTO test (id, project_id, name, script, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        )
        .bind(&t.id)
        .bind(&t.project_id)
        .bind(&t.name)
        .bind(&t.script)
        .bind(&t.created_at)
        .bind(&t.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(t)
    }

    /// Fetch a single test.
    pub async fn get_test(&self, id: &str) -> Result<Option<Test>, anyhow::Error> {
        let test = sqlx::query_as::<_, Test>("SELECT * FROM test WHERE id = ?1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(test)
    }

    /// Update a test's fields.
    pub async fn update_test(&self, test: &Test) -> Result<(), anyhow::Error> {
        let now = chrono::Utc::now().to_rfc3339();
        sqlx::query("UPDATE test SET name = ?1, script = ?2, updated_at = ?3 WHERE id = ?4")
            .bind(&test.name)
            .bind(&test.script)
            .bind(&now)
            .bind(&test.id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Delete a test.
    pub async fn delete_test(&self, id: &str) -> Result<(), anyhow::Error> {
        sqlx::query("DELETE FROM test WHERE id = ?1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// List all tests for a project.
    pub async fn list_tests(&self, project_id: &str) -> Result<Vec<Test>, anyhow::Error> {
        let tests =
            sqlx::query_as::<_, Test>("SELECT * FROM test WHERE project_id = ?1 ORDER BY name ASC")
                .bind(project_id)
                .fetch_all(&self.pool)
                .await?;

        Ok(tests)
    }

    // -----------------------------------------------------------------------
    // Environment
    // -----------------------------------------------------------------------

    /// Insert a new environment.
    pub async fn insert_environment(
        &self,
        environment: &Environment,
    ) -> Result<Environment, anyhow::Error> {
        let mut e = environment.clone();
        e.updated_at = chrono::Utc::now().to_rfc3339();

        sqlx::query(
            "INSERT INTO environment (id, project_id, name, variables, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        )
        .bind(&e.id)
        .bind(&e.project_id)
        .bind(&e.name)
        .bind(&e.variables)
        .bind(&e.created_at)
        .bind(&e.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(e)
    }

    /// Fetch a single environment.
    pub async fn get_environment(&self, id: &str) -> Result<Option<Environment>, anyhow::Error> {
        let env = sqlx::query_as::<_, Environment>("SELECT * FROM environment WHERE id = ?1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(env)
    }

    /// Update an environment.
    pub async fn update_environment(&self, environment: &Environment) -> Result<(), anyhow::Error> {
        let now = chrono::Utc::now().to_rfc3339();
        sqlx::query(
            "UPDATE environment SET name = ?1, variables = ?2, updated_at = ?3 WHERE id = ?4",
        )
        .bind(&environment.name)
        .bind(&environment.variables)
        .bind(&now)
        .bind(&environment.id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Delete an environment.
    pub async fn delete_environment(&self, id: &str) -> Result<(), anyhow::Error> {
        sqlx::query("DELETE FROM environment WHERE id = ?1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// List all environments for a project.
    pub async fn list_environments(
        &self,
        project_id: &str,
    ) -> Result<Vec<Environment>, anyhow::Error> {
        let envs = sqlx::query_as::<_, Environment>(
            "SELECT * FROM environment WHERE project_id = ?1 ORDER BY name ASC",
        )
        .bind(project_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(envs)
    }

    // -----------------------------------------------------------------------
    // ExecutionHistory
    // -----------------------------------------------------------------------

    /// Record a new execution-history entry.
    pub async fn insert_history(
        &self,
        history: &ExecutionHistory,
    ) -> Result<ExecutionHistory, anyhow::Error> {
        sqlx::query(
            "INSERT INTO execution_history
             (id, project_id, request_id, test_id, run_id,
              method, url, status, duration_ms,
              request_headers, response_headers, response_body, response_size,
              error_message, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
        )
        .bind(&history.id)
        .bind(&history.project_id)
        .bind(&history.request_id)
        .bind(&history.test_id)
        .bind(&history.run_id)
        .bind(&history.method)
        .bind(&history.url)
        .bind(history.status)
        .bind(history.duration_ms)
        .bind(&history.request_headers)
        .bind(&history.response_headers)
        .bind(&history.response_body)
        .bind(history.response_size)
        .bind(&history.error_message)
        .bind(&history.created_at)
        .execute(&self.pool)
        .await?;

        Ok(history.clone())
    }

    /// Update an existing history row with response data (called after the HTTP
    /// call completes).
    pub async fn update_history_response(
        &self,
        id: &str,
        status: u16,
        duration_ms: u64,
        request_headers: Option<&str>,
        response_headers: Option<&str>,
        response_body: Option<&[u8]>,
        response_size: usize,
        error_message: Option<&str>,
    ) -> Result<(), anyhow::Error> {
        sqlx::query(
            "UPDATE execution_history SET
                status = ?1, duration_ms = ?2,
                request_headers = ?3, response_headers = ?4,
                response_body = ?5, response_size = ?6,
                error_message = ?7
             WHERE id = ?8",
        )
        .bind(status as i64)
        .bind(duration_ms as i64)
        .bind(request_headers)
        .bind(response_headers)
        .bind(response_body)
        .bind(response_size as i64)
        .bind(error_message)
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Get a single history row.
    pub async fn get_history(&self, id: &str) -> Result<Option<ExecutionHistory>, anyhow::Error> {
        let row =
            sqlx::query_as::<_, ExecutionHistory>("SELECT * FROM execution_history WHERE id = ?1")
                .bind(id)
                .fetch_optional(&self.pool)
                .await?;

        Ok(row)
    }

    /// List history for a project, most recent first, with an optional limit.
    pub async fn list_history(
        &self,
        project_id: &str,
        limit: Option<i64>,
    ) -> Result<Vec<ExecutionHistory>, anyhow::Error> {
        let limit_val = limit.unwrap_or(50);

        let rows = sqlx::query_as::<_, ExecutionHistory>(
            "SELECT * FROM execution_history WHERE project_id = ?1 ORDER BY created_at DESC LIMIT ?2",
        )
        .bind(project_id)
        .bind(limit_val)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    /// List history for a specific request, most recent first.
    pub async fn list_history_for_request(
        &self,
        request_id: &str,
        limit: Option<i64>,
    ) -> Result<Vec<ExecutionHistory>, anyhow::Error> {
        let limit_val = limit.unwrap_or(20);

        let rows = sqlx::query_as::<_, ExecutionHistory>(
            "SELECT * FROM execution_history WHERE request_id = ?1 ORDER BY created_at DESC LIMIT ?2",
        )
        .bind(request_id)
        .bind(limit_val)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    /// Delete execution history older than the given ISO-8601 timestamp.
    pub async fn prune_history_older_than(
        &self,
        project_id: &str,
        older_than_iso: &str,
    ) -> Result<u64, anyhow::Error> {
        let result =
            sqlx::query("DELETE FROM execution_history WHERE project_id = ?1 AND created_at < ?2")
                .bind(project_id)
                .bind(older_than_iso)
                .execute(&self.pool)
                .await?;

        Ok(result.rows_affected())
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    async fn fixture() -> Store {
        Store::open_in_memory().await.expect("in-memory store")
    }

    #[tokio::test]
    async fn insert_and_get_project() {
        let store = fixture().await;
        let p = store
            .insert_project(&Project::new("Test Project"))
            .await
            .unwrap();
        assert!(!p.id.is_empty());
        assert_eq!(p.name, "Test Project");

        let fetched = store.get_project(&p.id).await.unwrap().unwrap();
        assert_eq!(fetched.name, "Test Project");
    }

    #[tokio::test]
    async fn update_project() {
        let store = fixture().await;
        let mut p = store.insert_project(&Project::new("Name A")).await.unwrap();
        p.name = "Name B".into();
        store.update_project(&p).await.unwrap();

        let fetched = store.get_project(&p.id).await.unwrap().unwrap();
        assert_eq!(fetched.name, "Name B");
    }

    #[tokio::test]
    async fn list_projects() {
        let store = fixture().await;
        store.insert_project(&Project::new("P1")).await.unwrap();
        store.insert_project(&Project::new("P2")).await.unwrap();

        let list = store.list_projects().await.unwrap();
        assert_eq!(list.len(), 2);
    }

    #[tokio::test]
    async fn delete_project_cascade() {
        let store = fixture().await;
        let project = store.insert_project(&Project::new("P")).await.unwrap();
        let folder = store
            .insert_folder(&Folder::new(&project.id, "F"))
            .await
            .unwrap();
        let req = store
            .insert_request(&Request::new(
                &project.id,
                "R",
                "GET",
                "https://example.com",
            ))
            .await
            .unwrap();

        store.delete_project(&project.id).await.unwrap();

        assert!(store.get_project(&project.id).await.unwrap().is_none());
        assert!(store.get_folder(&folder.id).await.unwrap().is_none());
        assert!(store.get_request(&req.id).await.unwrap().is_none());
    }

    #[tokio::test]
    async fn request_crud() {
        let store = fixture().await;
        let project = store.insert_project(&Project::new("P")).await.unwrap();

        // insert
        let req = store
            .insert_request(&Request::new(
                &project.id,
                "Get index",
                "GET",
                "https://httpbin.org/get",
            ))
            .await
            .unwrap();
        assert_eq!(req.name, "Get index");

        // update
        let mut updated = req.clone();
        updated.name = "Updated name".into();
        updated.url = "https://example.com".into();
        store.update_request(&updated).await.unwrap();

        let fetched = store.get_request(&req.id).await.unwrap().unwrap();
        assert_eq!(fetched.name, "Updated name");
        assert_eq!(fetched.url, "https://example.com");

        // delete
        store.delete_request(&req.id).await.unwrap();
        assert!(store.get_request(&req.id).await.unwrap().is_none());
    }

    #[tokio::test]
    async fn request_list_by_folder() {
        let store = fixture().await;
        let project = store.insert_project(&Project::new("P")).await.unwrap();
        let folder = store
            .insert_folder(&Folder::new(&project.id, "F"))
            .await
            .unwrap();

        store
            .insert_request(&{
                let mut r = Request::new(&project.id, "A", "GET", "/a");
                r.folder_id = Some(folder.id.clone());
                r.sort_order = 1;
                r
            })
            .await
            .unwrap();
        store
            .insert_request(&{
                let mut r = Request::new(&project.id, "B", "POST", "/b");
                r.folder_id = Some(folder.id.clone());
                r.sort_order = 0;
                r
            })
            .await
            .unwrap();
        // unfiled
        store
            .insert_request(&Request::new(&project.id, "C", "DELETE", "/c"))
            .await
            .unwrap();

        let in_folder = store
            .list_requests(&project.id, Some(&folder.id))
            .await
            .unwrap();
        assert_eq!(in_folder.len(), 2);
        assert_eq!(in_folder[0].name, "B"); // sort_order 0
        assert_eq!(in_folder[1].name, "A"); // sort_order 1

        let all = store.list_requests(&project.id, None).await.unwrap();
        assert_eq!(all.len(), 3);
    }

    #[tokio::test]
    async fn test_crud() {
        let store = fixture().await;
        let project = store.insert_project(&Project::new("P")).await.unwrap();

        let test = store
            .insert_test(&Test::new(&project.id, "My test"))
            .await
            .unwrap();
        assert_eq!(test.name, "My test");

        let mut updated = test.clone();
        updated.name = "Renamed".into();
        updated.script = "log(\"hello\")".into();
        store.update_test(&updated).await.unwrap();

        let fetched = store.get_test(&test.id).await.unwrap().unwrap();
        assert_eq!(fetched.name, "Renamed");
        assert_eq!(fetched.script, "log(\"hello\")");

        store.delete_test(&test.id).await.unwrap();
        assert!(store.get_test(&test.id).await.unwrap().is_none());
    }

    #[tokio::test]
    async fn environment_crud() {
        let store = fixture().await;
        let project = store.insert_project(&Project::new("P")).await.unwrap();

        let env = store
            .insert_environment(&Environment::new(&project.id, "dev"))
            .await
            .unwrap();
        assert_eq!(env.name, "dev");

        let mut updated = env.clone();
        updated.variables = r#"{"BASE_URL":"https://dev.example.com"}"#.into();
        store.update_environment(&updated).await.unwrap();

        let fetched = store.get_environment(&env.id).await.unwrap().unwrap();
        assert_eq!(
            fetched.variables,
            r#"{"BASE_URL":"https://dev.example.com"}"#
        );

        let list = store.list_environments(&project.id).await.unwrap();
        assert_eq!(list.len(), 1);
    }

    #[tokio::test]
    async fn execution_history_insert_and_update() {
        let store = fixture().await;
        let project = store.insert_project(&Project::new("P")).await.unwrap();
        let req = store
            .insert_request(&Request::new(
                &project.id,
                "R",
                "GET",
                "https://example.com",
            ))
            .await
            .unwrap();

        let run_id = uuid::Uuid::new_v4().to_string();
        let hist = store
            .insert_history(&ExecutionHistory::new(
                &project.id,
                Some(req.id.clone()),
                None,
                &run_id,
                "GET",
                "https://example.com",
                0,
                0,
            ))
            .await
            .unwrap();

        // Now simulate a response coming back
        store
            .update_history_response(
                &hist.id,
                200,
                123,
                Some(r#"{"accept":"*/*"}"#),
                Some(r#"{"content-type":"application/json"}"#),
                Some(b"{\"ok\":true}"),
                10,
                None,
            )
            .await
            .unwrap();

        let updated = store.get_history(&hist.id).await.unwrap().unwrap();
        assert_eq!(updated.status, 200);
        assert_eq!(updated.duration_ms, 123);
        assert_eq!(updated.response_body.unwrap(), b"{\"ok\":true}");
    }
}
