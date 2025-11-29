CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    runner_id INTEGER DEFAULT NULL,
    email TEXT NOT NULL UNIQUE,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL DEFAULT '',
    full_name TEXT NOT NULL,
    hometown_id INTEGER REFERENCES towns(id),
    image_url TEXT NOT NULL,
    role TEXT NOT NULL DEFAULT 'user',
    verified BOOLEAN NOT NULL DEFAULT 0,
    locked BOOLEAN NOT NULL DEFAULT 0,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_runner_id ON users(runner_id);
