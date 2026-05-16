//! SQLite schema creation / migration. The migration runs automatically when a
//! [`crate::store::Store`] is opened. Uses `CREATE TABLE IF NOT EXISTS` so it's
//! safe to call on every open.

/// Returns the SQL statements needed to bring a fresh (or outdated) database
/// up to the current schema. Statements are executed in order.
pub fn migration_statements() -> Vec<&'static str> {
    vec![
        // ── Project ──────────────────────────────────────────────────────
        "CREATE TABLE IF NOT EXISTS project (
            id          TEXT PRIMARY KEY NOT NULL,
            name        TEXT NOT NULL,
            description TEXT NOT NULL DEFAULT '',
            base_env    TEXT NOT NULL DEFAULT '{}',
            created_at  TEXT NOT NULL,
            updated_at  TEXT NOT NULL
        );",
        // ── Folder ───────────────────────────────────────────────────────
        "CREATE TABLE IF NOT EXISTS folder (
            id          TEXT PRIMARY KEY NOT NULL,
            project_id  TEXT NOT NULL REFERENCES project(id) ON DELETE CASCADE,
            parent_id   TEXT REFERENCES folder(id) ON DELETE CASCADE,
            name        TEXT NOT NULL,
            sort_order  INTEGER NOT NULL DEFAULT 0,
            created_at  TEXT NOT NULL,
            updated_at  TEXT NOT NULL
        );",
        // ── Request ──────────────────────────────────────────────────────
        "CREATE TABLE IF NOT EXISTS request (
            id                TEXT PRIMARY KEY NOT NULL,
            project_id        TEXT NOT NULL REFERENCES project(id) ON DELETE CASCADE,
            folder_id         TEXT REFERENCES folder(id) ON DELETE SET NULL,
            name              TEXT NOT NULL,
            method            TEXT NOT NULL,
            url               TEXT NOT NULL,
            summary           TEXT NOT NULL DEFAULT '',
            description       TEXT NOT NULL DEFAULT '',
            tags              TEXT NOT NULL DEFAULT '[]',
            headers           TEXT NOT NULL DEFAULT '{}',
            query_params      TEXT NOT NULL DEFAULT '{}',
            body_type         TEXT,
            body_content      TEXT,
            timeout_secs      INTEGER NOT NULL DEFAULT 30,
            follow_redirects  INTEGER NOT NULL DEFAULT 1,
            verify_tls        INTEGER NOT NULL DEFAULT 1,
            sort_order        INTEGER NOT NULL DEFAULT 0,
            created_at        TEXT NOT NULL,
            updated_at        TEXT NOT NULL
        );",
        // ── Test ─────────────────────────────────────────────────────────
        "CREATE TABLE IF NOT EXISTS test (
            id          TEXT PRIMARY KEY NOT NULL,
            project_id  TEXT NOT NULL REFERENCES project(id) ON DELETE CASCADE,
            name        TEXT NOT NULL,
            script      TEXT NOT NULL DEFAULT '',
            created_at  TEXT NOT NULL,
            updated_at  TEXT NOT NULL
        );",
        // ── Environment ──────────────────────────────────────────────────
        "CREATE TABLE IF NOT EXISTS environment (
            id          TEXT PRIMARY KEY NOT NULL,
            project_id  TEXT NOT NULL REFERENCES project(id) ON DELETE CASCADE,
            name        TEXT NOT NULL,
            variables   TEXT NOT NULL DEFAULT '{}',
            created_at  TEXT NOT NULL,
            updated_at  TEXT NOT NULL
        );",
        // ── ExecutionHistory ─────────────────────────────────────────────
        "CREATE TABLE IF NOT EXISTS execution_history (
            id               TEXT PRIMARY KEY NOT NULL,
            project_id       TEXT NOT NULL REFERENCES project(id) ON DELETE CASCADE,
            request_id       TEXT REFERENCES request(id) ON DELETE SET NULL,
            test_id          TEXT REFERENCES test(id) ON DELETE SET NULL,
            run_id           TEXT NOT NULL,
            method           TEXT NOT NULL,
            url              TEXT NOT NULL,
            status           INTEGER NOT NULL DEFAULT 0,
            duration_ms      INTEGER NOT NULL DEFAULT 0,
            request_headers  TEXT,
            response_headers TEXT,
            response_body    BLOB,
            response_size    INTEGER NOT NULL DEFAULT 0,
            error_message    TEXT,
            created_at       TEXT NOT NULL
        );",
        // ── Indexes ──────────────────────────────────────────────────────
        "CREATE INDEX IF NOT EXISTS idx_folder_project ON folder(project_id);",
        "CREATE INDEX IF NOT EXISTS idx_folder_parent ON folder(parent_id);",
        "CREATE INDEX IF NOT EXISTS idx_request_project ON request(project_id);",
        "CREATE INDEX IF NOT EXISTS idx_request_folder ON request(folder_id);",
        "CREATE INDEX IF NOT EXISTS idx_request_sort ON request(folder_id, sort_order);",
        "CREATE INDEX IF NOT EXISTS idx_test_project ON test(project_id);",
        "CREATE INDEX IF NOT EXISTS idx_env_project ON environment(project_id);",
        "CREATE INDEX IF NOT EXISTS idx_history_project ON execution_history(project_id);",
        "CREATE INDEX IF NOT EXISTS idx_history_run ON execution_history(run_id);",
        "CREATE INDEX IF NOT EXISTS idx_history_request ON execution_history(request_id);",
    ]
}
