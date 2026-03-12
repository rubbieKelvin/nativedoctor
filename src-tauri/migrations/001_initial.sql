CREATE TABLE IF NOT EXISTS recent_projects (
    path TEXT PRIMARY KEY,
    name TEXT,
    opened_at INTEGER NOT NULL
);
