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
CREATE TABLE IF NOT EXISTS `includes` (
	`including_folder`	INTEGER,
	`included_folder`	INTEGER,
	FOREIGN KEY(`including_folder`) REFERENCES `folders`(`id`),
	FOREIGN KEY(`included_folder`) REFERENCES `folders`(`id`),
	PRIMARY KEY(`including_folder`,`included_folder`)
);
CREATE TABLE IF NOT EXISTS `holds` (
	`held_folder`	INTEGER,
	`holded_album`		INTEGER,
	FOREIGN KEY(`holded_album`) REFERENCES `albums`(`id`),
	FOREIGN KEY(`held_folder`) REFERENCES `folders`(`id`),
	PRIMARY KEY(`held_folder`,`holded_album`)
);
CREATE TABLE IF NOT EXISTS `folders` (
	`id`	INTEGER NOT NULL UNIQUE,
	`name`	INTEGER,
	PRIMARY KEY(`id` AUTOINCREMENT)
);
CREATE TABLE IF NOT EXISTS `contains` (
	`containing_album`	INTEGER,
	`contained_photo`	INTEGER,
	FOREIGN KEY(`contained_photo`) REFERENCES `photos`(`id`),
	FOREIGN KEY(`containing_album`) REFERENCES `albums`(`id`),
	PRIMARY KEY(`containing_album`,`contained_photo`)
);
CREATE TABLE IF NOT EXISTS `albums` (
	`id`	INTEGER NOT NULL UNIQUE,
	`name`	TEXT,
	PRIMARY KEY(`id` AUTOINCREMENT)
);

