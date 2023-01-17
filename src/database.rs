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
use super::DATABASE_FILENAME;
use super::Error;
use crate::element::ElementDatabase;

use std::path::{Path, PathBuf};
use rusqlite::{Connection};

static DATABASE_SQL: &str = include_str!("../database.sql");

pub struct Database
{
	pub path: PathBuf,
	pub connection: Connection,
}

impl Database
{
	/// Creates a database object, and returns it with a open connection
	pub(crate) fn new<P: AsRef<Path>>(path: P) -> Result<Self, Error>
	{
		match Connection::open(path.as_ref().join(DATABASE_FILENAME))
		{
			Ok(c) =>
			{
				return Ok(Database
				   {
						path: path.as_ref().join(DATABASE_FILENAME),
						connection: c,
				   });
			}
			Err(_why) => return Err(Error::Other)
		};
	}

	/// Create the database object and file, and inserts the main structure
	pub(crate) fn create<P: AsRef<Path>>(path: P) -> Result<Self, Error>
	{
		let db = Self::new(path)?;
		match db.connection.execute_batch(DATABASE_SQL)
		{
			Ok(_) =>
			{
				return Ok(db);
			},
			Err(_why) => return Err(Error::Other)
		}
	}
}

impl Database
{
	/// Inserts an element into the database
	pub(crate) fn insert(&self, object: &dyn ElementDatabase) -> Result<u32, Error>
	{
		object.insert_into(self)
	}
	/// Gets an element from the database with its id
	pub(crate) fn from_id(&self, object: &mut dyn ElementDatabase, id: u32) -> Result<(), Error>
	{
		object.from_id(self, id)
	}
	/// Deletes an element f rom the database with its self.id
	pub(crate) fn delete(&self, object: &dyn ElementDatabase) -> Result<(), Error>
	{
		object.delete(self)
	}
}
