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

use crate::element::album::Album;
use crate::element::traits::ElementDatabase;
use crate::element::traits::ElementFilesystem;
use crate::Database;
use crate::Filesystem;
use crate::OsplError;
use crate::element::traits::ElementListing;
use crate::element::traits::InsideElementListing;

use chrono::naive::NaiveDateTime;

/// Structure containing a replica of sqlite data
#[derive(Debug)]
pub struct Collection
{
	id:						u32,
	creation_datetime:		Option<NaiveDateTime>,
	modification_datetime:	Option<NaiveDateTime>,
	name:					String,
	comment:				String,
}
impl Default for Collection
{
	fn default() -> Self
	{
		Collection::new()
	}
}

// Constructors
impl Collection
{
	/// Returns an empty Collection element
	pub fn new() -> Self
	{
		Collection
		{
			id: 0,
			creation_datetime: None,
			modification_datetime: None,
			name: String::from(""),
			comment: String::from(""),
		}
	}

	/// Returns a collection element filled with name and dates
	pub fn new_with_name(name: &str, comment: &str) -> Self
	{
		Collection
		{
			id:						0,
			creation_datetime:		Some(chrono::offset::Local::now().naive_local()),
			modification_datetime:	Some(chrono::offset::Local::now().naive_local()),
			name:					String::from(name),
			comment:				String::from(comment),
		}
	}

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

	/* TODO: Implement a from_folder function for importing. */
}

impl ElementDatabase for Collection
{
	fn delete(&self, db: &Database) -> Result<(), OsplError>
	{
		db.connection.execute("DELETE FROM collections WHERE id = ?1", [&self.id])?;
		Ok(())
	}

	fn insert_into(&self, db: &Database) -> Result<u32, OsplError>
	{
		db.connection.execute("INSERT INTO collections (name, comment, creation_datetime, modification_datetime) VALUES (?1, ?2, ?3, ?4)",
		(&self.name, &self.comment, &self.creation_datetime, &self.modification_datetime))?;
		Ok(db.connection.last_insert_rowid() as u32)
	}

	fn rename(&self, db: &Database, new_name: &str) -> Result<(), OsplError>
	{
		db.connection.execute("UPDATE collections SET name = ?1 WHERE id = ?2", (new_name, &self.id))?;
		Ok(())
	}

	fn load_from_id(&mut self, db: &Database, id: u32) -> Result<(), OsplError>
	{
		// fill self with the Collection table from the database with the id
		let mut stmt = db.connection.prepare("SELECT * FROM collections WHERE id = ?1")?;
		let mut rows = stmt.query([&id])?;

		while let Some(row) = rows.next()?
		{
			self.id = row.get(0)?;
			self.name = row.get(1)?;
			self.comment = row.get(2)?;
			self.creation_datetime = row.get(3)?;
			self.modification_datetime = row.get(4)?;
		}
		if self.id == 0
		{
			return Err(OsplError::IoError(std::io::ErrorKind::NotFound));
		}
		Ok(())
	}
}

impl InsideElementListing<Album> for Collection
{
	fn list_inside(db: &Database, collection: u32)-> Result<Vec<Album>, OsplError>
	{
		let mut stmt = db.connection.prepare("SELECT * FROM albums WHERE collection = ?1")?;
		let mut rows = stmt.query([&collection])?;

		let mut albums = Vec::new();
		while let Some(row) = rows.next()?
		{
			let mut c = Collection::default();
			c.load_from_id(db, row.get(5)?)?;
			let album = Album
			{
				id: row.get(0)?,
				name: row.get(1)?,
				comment: row.get(2)?,
				creation_datetime: Some(row.get(3)?),
				modification_datetime: Some(row.get(4)?),
				collection: c,
			};
			albums.push(album);
		}	
		Ok(albums)
	}
}


impl ElementListing<Collection> for Collection
{
	fn list_all(db: &Database, _fs: &Filesystem) -> Result<Vec<Collection>, OsplError>
	{
		let mut stmt = db.connection.prepare("SELECT * FROM collections")?;
		let mut rows = stmt.query(())?;

		let mut collections = Vec::new();
		while let Some(row) = rows.next()?
		{
			let collection = Collection
			{
				id: row.get(0)?,
				name: row.get(1)?,
				comment: row.get(2)?,
				creation_datetime: Some(row.get(3)?),
				modification_datetime: Some(row.get(4)?),
			};
			collections.push(collection);
		}	
		Ok(collections)
	}
}


impl ElementFilesystem for Collection
{
	fn insert_into(&self, fs: &Filesystem) -> Result<(), OsplError>
	{
		let path = fs.collections_path().join(&self.name);
		std::fs::create_dir(path)?;
		Ok(())
	}

	fn remove_from(&self, fs: &Filesystem) -> Result<(), OsplError>
	{
		Ok(std::fs::remove_dir_all(fs.collections_path())?)
	}

	fn rename(&self, fs: &Filesystem, new_name: &str) -> Result<(), OsplError>
	{
		let path_old = fs.collections_path().join(self.name());
		let path_new = fs.collections_path().join(new_name);
		Ok(std::fs::rename(path_old, path_new)?)
	}
}
