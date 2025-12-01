CREATE TABLE counties (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL
);

INSERT INTO counties (id, name) VALUES
	(1, 'Fairfield'),
	(2, 'Hartford'),
	(3, 'Litchfield'),
	(4, 'Middlesex'),
	(5, 'New Haven'),
	(6, 'New London'),
	(7, 'Tolland'),
	(8, 'Windham');

CREATE INDEX idx_counties_name ON counties(name);
