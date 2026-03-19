-- Response: stores the full received response
CREATE TABLE IF NOT EXISTS response (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    status_code INTEGER NOT NULL,
    status_text TEXT,
    headers TEXT NOT NULL,
    body TEXT,
    content_type TEXT,
    content_length INTEGER,
    duration_ms INTEGER,
    created_at INTEGER NOT NULL
);

-- Requests: stores the full sent request and references its response
CREATE TABLE IF NOT EXISTS requests (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    method TEXT NOT NULL,
    url TEXT NOT NULL,
    path TEXT,
    query_string TEXT,
    headers TEXT NOT NULL,
    body TEXT,
    content_type TEXT,
    content_length INTEGER,
    created_at INTEGER NOT NULL,
    response_id INTEGER,
    FOREIGN KEY (response_id) REFERENCES response (id)
);
