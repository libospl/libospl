-- Table where each row represents a setting.
CREATE TABLE IF NOT EXISTS settings (
	name	TEXT NOT NULL UNIQUE,
	value	TEXT,
	PRIMARY KEY (name)
);
-- Table where each row represents a photo.
CREATE TABLE IF NOT EXISTS photos (
	id						INTEGER NOT NULL UNIQUE,
	filename				TEXT NOT NULL UNIQUE,
	hash					TEXT NOT NULL,
	thumbnail_hash			TEXT,
	import_datetime			DATETIME,
	-- Image information
	height					INTEGER,
	width					INTEGER,
	creation_datetime		DATETIME,
	format					TEXT,
	orientation				TEXT,
	rating					INTEGER DEFAULT 0,
	starred					INTEGER DEFAULT 0,
	-- Image metadata
	make					TEXT,
	model					TEXT,
	lens					TEXT,
	aperture				REAL,
	focal_length			REAL,
	exposure_time			TEXT,
	exposure_mode			INTEGER,
	sensitivity				INTEGER,
	flash					INTEGER,
	metering_mode			INTEGER,
	title					TEXT,
	comment					TEXT,
	-- Image position: TODO: Research this
	-- Other
	album					INTEGER,
	-- Key configuration
	FOREIGN KEY(album) REFERENCES albums(id),
	PRIMARY KEY(id AUTOINCREMENT)
);

-- Table where each row represents an album.
CREATE TABLE IF NOT EXISTS albums (
	id						INTEGER NOT NULL UNIQUE,
	name					TEXT NOT NULL UNIQUE,
	comment 				TEXT,
	creation_datetime		TEXT,
	modification_datetime	DATETIME,
	collection				INTEGER,
	FOREIGN KEY(collection) REFERENCES collections(id),
	PRIMARY KEY(id AUTOINCREMENT)
);

-- Table where each row represents a collection.
CREATE TABLE IF NOT EXISTS collections (
	id						INTEGER NOT NULL UNIQUE,
	name					TEXT NOT NULL UNIQUE,
	comment					TEXT,
	creation_datetime		TEXT,
	modification_datetime	DATETIME,
	PRIMARY KEY(id AUTOINCREMENT)
);

-- Table where each row represents a tag.
CREATE TABLE IF NOT EXISTS tags (
	id						INTEGER NOT NULL UNIQUE,
	name					TEXT NOT NULL UNIQUE,
	PRIMARY KEY(id AUTOINCREMENT)
);
-- Link table between photos and tags.
CREATE TABLE IF NOT EXISTS photos_tags_map (
	containing_tag			INTEGER NOT NULL,
	contained_photo			INTEGER NOT NULL,
	FOREIGN KEY(contained_photo) REFERENCES photos(id),
	FOREIGN KEY(containing_tag) REFERENCES tags(id)
);

