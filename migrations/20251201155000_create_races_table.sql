CREATE TABLE races (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    town_id INTEGER NOT NULL REFERENCES towns(id),
    name TEXT NOT NULL,
    miles REAL NOT NULL,
    start_date DATE NOT NULL,
    street_address TEXT,
    race_url TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(town_id, name, miles)
);

-- Update updated_at when there are changes
CREATE TRIGGER set_races_updated_at
AFTER UPDATE ON races
FOR EACH ROW
BEGIN
    UPDATE races
    SET updated_at = CURRENT_TIMESTAMP
    WHERE id = NEW.id;
END;

-- Races View
CREATE VIEW races_view AS
SELECT r.*, t.name AS town, t.county
FROM races r
LEFT JOIN towns_view t ON t.id = r.town_id;
