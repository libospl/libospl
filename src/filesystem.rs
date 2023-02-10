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

use crate::Error;
use crate::database::Database;
use crate::element::ElementFilesystem;

use super::DATABASE_FILENAME;

use std::path::{Path, PathBuf};

/// The Filesystem structure manages every file and directory in the library.
#[derive(Debug)]
pub struct Filesystem
{
	root_path: PathBuf,
	pictures_path: PathBuf,
	thumbnails_path: PathBuf,
	collections_path: PathBuf,
	database_path:	PathBuf,
}

impl Filesystem
{
	/// Creates a filesystem object, and returns it
	pub(crate) fn new<P: AsRef<Path>>(path: P) -> Result<Self, Error>
	{
		return Ok(Filesystem
			{
				root_path: path.as_ref().to_path_buf(),
				thumbnails_path: path.as_ref().join("thumbnails"),
				pictures_path: path.as_ref().join("pictures"),
				collections_path: path.as_ref().join("collections"),
				database_path: path.as_ref().join(DATABASE_FILENAME),
			});
	}

	/// Create the filesystem object and creates the main fs structure
	pub(crate) fn create<P: AsRef<Path>>(path: P) -> Result<Self, Error>
	{
		let fs = Self::new(path)?;
		std::fs::create_dir(&fs.thumbnails_path)?;
		std::fs::create_dir(&fs.pictures_path)?;
		std::fs::create_dir(&fs.collections_path)?;
		Database::create(&fs.database_path)?;
		Ok(fs)
	}
}

impl Filesystem
{
	/// Returns the path on filesystem of the library folder
	pub fn root_path(&self) -> PathBuf
	{
		self.root_path.clone()
	}

	/// Returns the path on filesystem to the pictures path in the library
	pub fn pictures_path(&self) -> PathBuf
	{
		self.pictures_path.to_path_buf()
	}

	/// Returns the path on filesystem to the thumbnails path in the library
	pub fn thumbnails_path(&self) -> PathBuf
	{
		self.thumbnails_path.to_path_buf()
	}

	/// Returns the path on filesystem to the collections path in the library
	pub fn collections_path(&self) -> PathBuf
	{
		self.collections_path.to_path_buf()
	}

	/// Returns the path on filesystem of the database file
	pub fn database_path(&self) -> PathBuf
	{
		self.database_path.clone()
	}
}

impl Filesystem
{
	/// Inserts the element into the library filesystem
	///
	/// If fs.insert(object) is called, it will call object.insert_into(Filesystem struct)
	pub(crate) fn insert(&self, object: &dyn ElementFilesystem) -> Result<(), Error>
	{
		object.insert_into(self)
	}

	/// Rename the element in the library filesystem
	///
	/// If fs.rename(object) is called it will call object.rename(Filesystem struct)
	pub(crate) fn rename(&self, object: &dyn ElementFilesystem, new_name: &str) -> Result<(), Error>
	{
		object.rename(self, new_name)
	}

	/// Removes the element from the library filesystem
	///
	/// If fs.remove(object) is called, it will call object.remove_from(Filesystem struct)
	pub(crate) fn remove(&self, object: &dyn ElementFilesystem) -> Result<(), Error>
	{
		object.remove_from(self)
	}
}
