/*	libospl - Open Source Collection Library
	an opensource and multiplateform Collection library management that can be used
	to store and sort all your Collections.
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

use crate::element::ElementDatabase;
use crate::element::ElementFilesystem;
use crate::Database;
use crate::Filesystem;
use crate::Error;

use chrono::naive::NaiveDateTime;

/// Structure containing a replica of sqlite data
pub struct Collection
{
	pub id:					u32,
	creation_datetime:		Option<NaiveDateTime>,
	modification_datetime:	Option<NaiveDateTime>,
	name:					String,
	comment:				String,
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

	pub fn name (&self) -> String
	{
		self.name.clone()
	}
	
	pub fn comment (&self) -> String
	{
		self.comment.clone()
	}

	/* TODO: Implement a from_folder function for importing. */
}

impl ElementDatabase for Collection
{
	fn delete(&self, db: &Database) -> Result<(), Error>
	{
		match db.connection.execute("DELETE FROM collections WHERE id = ?1", &[&self.id])
		{
			Ok(_) => Ok(()),
			Err(_) => return Err(Error::Other)
		}
	}

	fn insert_into(&self, db: &Database) -> Result<u32, Error>
	{
		match db.connection.execute("INSERT INTO collections (name, comment, creation_datetime, modification_datetime) VALUES (?1, ?2, ?3, ?4)",
		(&self.name, &self.comment, &self.creation_datetime, &self.modification_datetime))
		{
			Ok(_) => Ok(db.connection.last_insert_rowid() as u32),
			Err(_) => return Err(Error::Other)
		}
	}

	fn from_id(&mut self, db: &Database, id: u32) -> Result<(), Error>
	{
		// fill self with the Collection table from the database with the id
		let mut stmt = db.connection.prepare("SELECT * FROM collections WHERE id = ?1")?;
		let mut rows = stmt.query(&[&id])?;

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
			return Err(Error::NotFound);
		}
		Ok(())
	}
}

impl ElementFilesystem for Collection
{
	fn insert_into (&self, fs: &Filesystem) -> Result<(), Error> 
	{
		let path = fs.collections_path().join(&self.name);
		std::fs::create_dir(&path)?;
		Ok(())
	}

	fn remove_from (&self, _fs: &Filesystem) -> Result<(), Error> 
	{
		unimplemented!()
	}
}
