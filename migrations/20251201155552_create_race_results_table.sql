CREATE TABLE race_results (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL REFERENCES users(id),
    race_id INTEGER NOT NULL REFERENCES races(id),
    notes TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(user_id, race_id)
);

-- Update updated_at when there are changes
CREATE TRIGGER set_race_results_updated_at
AFTER UPDATE ON race_results
FOR EACH ROW
BEGIN
    UPDATE race_results
    SET updated_at = CURRENT_TIMESTAMP
    WHERE id = NEW.id;
END;
