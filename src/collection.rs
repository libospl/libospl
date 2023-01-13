/*	libospl - Open Source Photo Library
	an opensource and multiplateform photo library management that can be used
	to store and sort all your photos.
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
use crate::Database;
use chrono::naive::NaiveDateTime;
use crate::Error;

/// Structure containing a replica of sqlite data
#[derive(Debug)]
#[allow(dead_code)]
pub struct Collection
{
	pub id:					u32,
	name:					String,
	comment:				String,
	creation_datetime:		Option<NaiveDateTime>,
	modification_datetime:	Option<NaiveDateTime>
}

impl Collection
{
	/// Returns an empty Photo element
	pub fn new(name: String, comment: String) -> Self
	{
		Collection
		{
			id:						0,
			name:					name,
			comment:				comment,
			creation_datetime:		None,
			modification_datetime:	None
		}
	}
}

impl ElementDatabase for Collection
{
	fn insert_into(&self, db: &Database) -> Result<u32, Error>
	{
		/* Create a table with the name and comment passed along. */
		match db.connection.execute("INSERT INTO collections (name, comment) VALUES (?1, ?2)", (&self.name, &self.comment))
		{
			Ok(_) => Ok(db.connection.last_insert_rowid() as u32),
    		Err(_) => return Err(Error::Other)
		}
	}
	fn delete(&self, db: &Database) -> Result<(), Error>
	{
		match db.connection.execute("DELETE FROM collections WHERE id = ?1", &[&self.id])
		{
			Ok(_) => Ok(()),
			Err(_) => return Err(Error::Other)
		}
	}
	fn from_id(&mut self, db: &Database, id: u32) -> Result<(), Error>
	{
		// fill self with the photo table from the database with the id
		let mut stmt = db.connection.prepare("SELECT * FROM collections WHERE id = ?1")?;
		let mut rows = stmt.query(&[&id])?;
		while let Some(row) = rows.next()?
		{
			self.id = row.get(0)?;
			self.name = row.get(1)?;
			self.comment = row.get(2)?;
		}
		if self.id == 0
		{
			return Err(Error::NotFound);
		}
		Ok(())
	}
}
