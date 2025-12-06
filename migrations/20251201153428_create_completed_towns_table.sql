CREATE TABLE completed_towns (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    town_id INTEGER NOT NULL REFERENCES towns(id) ON DELETE CASCADE,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(user_id, town_id)
);

-- Update updated_at when there are changes
CREATE TRIGGER set_completed_towns_updated_at
AFTER UPDATE ON completed_towns
FOR EACH ROW
BEGIN
    UPDATE completed_towns
    SET updated_at = CURRENT_TIMESTAMP
    WHERE id = NEW.id;
END;

-- Completed Town View
CREATE VIEW completed_towns_view AS
SELECT ct.*, t.name, t.county, t.is_elusive
FROM completed_towns ct
LEFT JOIN towns_view t ON t.id = ct.town_id;
