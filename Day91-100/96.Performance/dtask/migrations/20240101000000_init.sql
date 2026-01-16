CREATE TABLE tasks (
    id TEXT PRIMARY KEY,
    payload TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    status INTEGER NOT NULL
);
