/*	libospl - Open Source Photo Library
	an opensource and multiplateform Photo library management that can be used
	to store and sort all your Photos.
	Copyright (C) 2019-2022 Angelo Frangione

	This program is free software; you can redistribute it and/or modify
	it under the terms of the GNU General Public License as published by
	the Free Software Foundation; either version 2 of the License, or
	(at your option) any later version.

	This program is distributed in the hope that it will be useful,
	but WITHOUT ANY WARRANTY; without even the implied warranty of
	MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
	GNU General Public License for more details.

	You should have received a copy of the GNU General Public License along
	with this program; if not, write to the Free Software Foundation, Inc.,
	51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
*/

use crate::element::collection::Collection;
use crate::element::traits::InsideElementListing;
use crate::element::photo::Photo;
use crate::element::traits::ElementDatabase;
use crate::element::traits::ElementFilesystem;
use crate::Database;
use crate::Filesystem;
use crate::OsplError;

use std::path::PathBuf;
use chrono::naive::NaiveDateTime;
use log::warn;

/// Structure containing a replica of sqlite data
#[derive(Debug)]
pub struct Album
{
	pub(crate) id:					u32,
	pub(crate) creation_datetime:		Option<NaiveDateTime>,
	pub(crate) modification_datetime:	Option<NaiveDateTime>,
	pub(crate) name:					String,
	pub(crate) comment:				String,
	pub(crate) collection:				Collection,
}

// Constructors
impl Album
{
	/// Returns an empty Album element
	pub fn new() -> Self
	{
		Album
		{
			id: 0,
			creation_datetime: None,
			modification_datetime: None,
			name: String::from(""),
			comment: String::from(""),
			collection: Collection::new(),
		}
	}

	/// Returns a album element filled with name and dates
	pub fn new_with_name(name: &str, comment: &str, collection: Collection) -> Self
	{
		Album
		{
			id:						0,
			creation_datetime:		Some(chrono::offset::Local::now().naive_local()),
			modification_datetime:	Some(chrono::offset::Local::now().naive_local()),
			name:					String::from(name),
			comment:				String::from(comment),
			collection,
		}
	}
}

// Getters and setters
impl Album
{
	pub fn set_id(&mut self, id: u32)
	{
		self.id = id;
	}

	pub fn id(&self) -> u32
	{
		self.id
	}

	pub fn name(&self) -> String
	{
		self.name.clone()
	}
	
	pub fn comment(&self) -> String
	{
		self.comment.clone()
	}

	pub fn collection_id(&self) -> u32
	{
		self.collection.id()
	}
}

// Internal helpers
impl Album
{
	fn get_collection_path(&self, fs: &Filesystem) -> PathBuf
	{
		fs.collections_path().join(self.collection.name())
	}

	fn get_full_path(&self, fs: &Filesystem) -> PathBuf
	{
		self.get_collection_path(fs).join(self.name())
	}
}

impl ElementDatabase for Album
{
	fn delete(&self, db: &Database) -> Result<(), OsplError>
	{
		db.connection.execute("DELETE FROM albums WHERE id = ?1", [&self.id])?;
		Ok(())
	}

	fn insert_into(&self, db: &Database) -> Result<u32, OsplError>
	{
		db.connection.execute("INSERT INTO albums (name, comment, creation_datetime, modification_datetime, collection) VALUES (?1, ?2, ?3, ?4, ?5)",
		(&self.name, &self.comment, &self.creation_datetime, &self.modification_datetime, self.collection.id()))?;
		Ok(db.connection.last_insert_rowid() as u32)
	}

	fn rename(&self, db: &Database, new_name: &str) -> Result<(), OsplError>
	{
		db.connection.execute("UPDATE albums SET name = ?1 WHERE id = ?2", (new_name, &self.id))?;
		Ok(())
	}

	fn from_id(&mut self, db: &Database, id: u32) -> Result<(), OsplError>
	{
		// fill self with the albums table from the database with the id
		let mut stmt = db.connection.prepare("SELECT * FROM albums WHERE id = ?1")?;
		let mut rows = stmt.query([&id])?;

		while let Some(row) = rows.next()?
		{
			self.id = row.get(0)?;
			self.name = row.get(1)?;
			self.comment = row.get(2)?;
			self.creation_datetime = row.get(3)?;
			self.modification_datetime = row.get(4)?;
			db.from_id(&mut self.collection, row.get(5)?)?;
		}
		
		if self.id == 0
		{
			return Err(OsplError::IoError(std::io::ErrorKind::NotFound));
		}
		Ok(())
	}
}

impl ElementFilesystem for Album
{
	fn insert_into(&self, fs: &Filesystem) -> Result<(), OsplError> 
	{
		std::fs::create_dir(self.get_full_path(fs))?;
		Ok(())
	}

	fn remove_from(&self, fs: &Filesystem) -> Result<(), OsplError>
	{
		Ok(std::fs::remove_dir_all(self.get_full_path(fs))?)
	}

	fn rename(&self, fs: &Filesystem, new_name: &str) -> Result<(), OsplError>
	{
		let path_old = self.get_collection_path(fs).join(self.name());
		let path_new = self.get_collection_path(fs).join(new_name);
		Ok(std::fs::rename(path_old, path_new)?)
	}
}

// Specific Filesystem functions
impl Album
{
	pub fn move_to(&self, fs: &Filesystem, collection: &Collection) -> Result<(), OsplError>
	{
		let path_old = self.get_collection_path(fs).join(self.name());
		let path_new = fs.collections_path().join(collection.name()).join(self.name());
		Ok(std::fs::rename(path_old, path_new)?)
	}

	pub fn add(&self, fs: &Filesystem, photo: &Photo) -> Result<(), OsplError>
	{
		let photo_path = fs.pictures_path().join(photo.get_filename());
		let link_path = fs.collections_path()
									.join(self.collection.name())
									.join(self.name())
									.join(photo.get_filename());
		if !link_path.exists()
		{
			std::fs::hard_link(photo_path, link_path)?;
		}
		Ok(())
	}
}

// Specific Database functions
impl Album
{
	pub fn assign_to(&self, db: &Database, collection: &Collection) -> Result<(), OsplError>
	{
		if self.collection.id() == collection.id()
		{
			warn!("This album (id:{}) is already assigned to this collection(id:{})", self.id, self.collection.id());
		}
		db.connection.execute("UPDATE albums SET collection = ?1 WHERE id = ?2", (collection.id(), &self.id))?;
		Ok(())
	}

	pub fn put(&self, db: &Database, photo: &Photo) -> Result<(), OsplError>
	{
		db.connection.execute("INSERT INTO photos_albums_map (containing_album, contained_photo) VALUES (?1, ?2)",
		(self.id(), photo.id))?;
		Ok(())
	}
}

impl InsideElementListing<Photo> for Album
{
	fn list_inside(db: &Database, id: u32) -> Result<Vec<Photo>, OsplError>
	{
		let mut stmt = db.connection.prepare("SELECT contained_photo FROM photos_albums_map WHERE containing_album = ?1")?;
		let mut rows = stmt.query([&id])?;
		let mut photos = Vec::new();
		while let Some(row) = rows.next()?
		{
			let mut photo = Photo::new();
			db.from_id(&mut photo, row.get(0)?)?;
			photos.push(photo);
		}
		Ok(photos)
	}
}
