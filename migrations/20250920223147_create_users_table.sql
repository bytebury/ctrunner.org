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

-- Update updated at when there are changes
CREATE TRIGGER set_users_updated_at
AFTER UPDATE ON users
FOR EACH ROW
BEGIN
    UPDATE users
    SET updated_at = CURRENT_TIMESTAMP
    WHERE id = NEW.id;
END;

-- User View
CREATE VIEW users_view AS
SELECT u.*, t.name AS hometown, t.county_id AS hometown_county_id, t.county AS hometown_county
FROM users u
LEFT JOIN towns_view t ON u.hometown_id = t.id;
