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

use super::OsplError;
use crate::element::traits::ElementDatabase;

use std::path::{Path};
use rusqlite::{Connection};

static DATABASE_SQL: &str = include_str!("../database.sql");

/// The database structure manages the connection to the db and every db entry.
#[derive(Debug)]
pub struct Database
{
	pub connection: Connection,
}

impl Database
{
	/// Creates a database object, and returns it with a open connection
	pub(crate) fn new<P: AsRef<Path>>(path: P) -> Result<Self, OsplError>
	{
		Ok(Database
		{
			connection: Connection::open(path.as_ref())?,
		})
	}

	/// Create the database object and file, and inserts the main structure
	pub(crate) fn create<P: AsRef<Path>>(path: P) -> Result<Self, OsplError>
	{
		let db = Self::new(path)?;
		db.connection.execute_batch(DATABASE_SQL)?;
		Ok(db)
	}
}

impl Database
{
	/// Inserts an element into the database
	///
	/// If db.insert(object) is called, it will call object.insert_into(database struct)
	pub(crate) fn insert(&self, object: &dyn ElementDatabase) -> Result<u32, OsplError>
	{
		object.insert_into(self)
	}

	/// Gets an element from the database with its id
	///
	/// If db.from_id(object) is called, it will call object.from_id(database struct)
	pub(crate) fn load_from_id(&self, object: &mut dyn ElementDatabase, id: u32) -> Result<(), OsplError>
	{
		object.load_from_id(self, id)
	}

	/// Rename the element in the database
	///
	/// If db.rename(object) is called it will call object.rename(database struct)
	pub(crate) fn rename(&self, object: &dyn ElementDatabase, new_name: &str) -> Result<(), OsplError>
	{
		object.rename(self, new_name)
	}

	/// Deletes an element f rom the database with its self.id
	///
	/// If db.delete(object) is called, it will call object.delete(database struct)
	pub(crate) fn delete(&self, object: &dyn ElementDatabase) -> Result<(), OsplError>
	{
		object.delete(self)
	}
}
