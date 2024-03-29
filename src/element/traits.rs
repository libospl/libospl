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

use crate::Database;
use crate::Filesystem;
use crate::OsplError;

/// Trait related to database interactions
pub trait ElementDatabase
{
	/// Insert the element into the given database
	fn insert_into(&self, db: &Database) -> Result<u32, OsplError>;
	/// Fill the element with data from database
	fn load_from_id(&mut self, db: &Database, id: u32) -> Result<(), OsplError>;
	/// Deletes the element from the database, the element must be loaded and have an id
	fn delete(&self, db: &Database) -> Result<(), OsplError>;
	/// Rename the element in the database
	fn rename(&self, db: &Database, new_name: &str) -> Result<(), OsplError>;
}

pub trait InsideElementListing<T>
{
	fn list_inside(db: &Database, id: u32)-> Result<Vec<T>, OsplError>;
}

pub trait ElementListing<T>
{
	fn list_all(db: &Database, fs: &Filesystem)-> Result<Vec<T>, OsplError>;
}

/// Trait related to filesystem interactions
pub trait ElementFilesystem
{
	/// Inserts the element into the library filesystem
	fn insert_into(&self, fs: &Filesystem) -> Result<(), OsplError>;
	/// Removes the element from the library filesystem
	fn remove_from(&self, fs: &Filesystem) -> Result<(), OsplError>;
	/// Rename the element on the library filesystem
	fn rename(&self, fs: &Filesystem, new_name: &str) -> Result<(), OsplError>;
}