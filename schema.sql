CREATE TABLE IF NOT EXISTS download_stats(
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	name TEXT NOT NULL,
	type TEXT NOT NULL CHECK(type IN ('plugins', 'bin', 'templates')),
	count INTEGER NOT NULL DEFAULT 0,
	last_downloaded_at DATETIME DEFAULT CURRENT_TIMESTAMP,
	UNIQUE(name)
);

ALTER TABLE download_stats DROP COLUMN last_downloaded_at;