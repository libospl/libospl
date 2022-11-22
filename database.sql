CREATE TABLE IF NOT EXISTS `settings` (
	`name`	TEXT NOT NULL UNIQUE,
	`value`	TEXT,
	PRIMARY KEY(`name`)
);
CREATE TABLE IF NOT EXISTS `photos` (
	`id`				INTEGER NOT NULL UNIQUE,
	`hash`				TEXT NOT NULL,
	`original_name`		TEXT NOT NULL,
	`new_name`			TEXT NOT NULL,
	`import_datetime`	TEXT,
	`random`			TEXT,
	`import_year`		INTEGER,
	`import_month`		INTEGER,
	`import_day`		INTEGER,
	`import_hour`		INTEGER,
	`import_minute`		INTEGER,
	`import_second`		INTEGER,
	`exif_height`		INTEGER,
	`exif_width`		INTEGER ,
	`exif_time`			TEXT,
	`exif_brand`		TEXT,
	`exif_peripheral`	TEXT,
	`fav`				INTEGER DEFAULT 0,
	PRIMARY KEY(`id` AUTOINCREMENT)
);
CREATE TABLE IF NOT EXISTS `folders` (
	`id`	INTEGER NOT NULL UNIQUE,
	`name`	INTEGER,
	PRIMARY KEY(`id` AUTOINCREMENT)
);
CREATE TABLE IF NOT EXISTS `collections` (
	`id`	INTEGER NOT NULL UNIQUE,
	`name`	TEXT,
	`albums` ARRAY,						-- TODO
	PRIMARY KEY(`id` AUTOINCREMENT)
);
CREATE TABLE IF NOT EXISTS `albums` (
	`id`	INTEGER NOT NULL UNIQUE,
	`name`	TEXT,
	`photos` ARRAY,						-- TODO
	PRIMARY KEY(`id` AUTOINCREMENT)
);
CREATE TABLE IF NOT EXISTS `tags` (
	`id`	INTEGER NOT NULL UNIQUE,
	`name`	TEXT,
	PRIMARY KEY(`id` AUTOINCREMENT)
);
