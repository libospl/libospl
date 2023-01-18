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

use crate::Directory;
use crate::Error;
use crate::element::ElementFilesystem;

use std::path::{Path, PathBuf};

pub struct Filesystem
{
	pub path: PathBuf,
	pictures_path: PathBuf,
	thumbnails_path: PathBuf,
	collections_path: PathBuf,
}

impl Filesystem
{
	/// Creates a filesystem object, and returns it
	pub(crate) fn new<P: AsRef<Path>>(path: P) -> Result<Self, Error>
	{
		return Ok(Filesystem
			{
				path: path.as_ref().to_path_buf(),
				thumbnails_path: path.as_ref().join("thumbnails"),
				pictures_path: path.as_ref().join("pictures"),
				collections_path: path.as_ref().join("collections"),
			});
	}

	/// Create the filesystem object and creates the main fs structure
	pub(crate) fn create<P: AsRef<Path>>(path: P) -> Result<Self, Error>
	{
		let fs = Self::new(path)?;

		Directory::from(&fs.thumbnails_path)?.create()?;
		Directory::from(&fs.pictures_path)?.create()?;
		Directory::from(&fs.collections_path)?.create()?;
		Ok(fs)
	}


}

impl Filesystem
{
	pub fn get_pictures_path(&self) -> PathBuf
	{
		self.pictures_path.to_path_buf()
	}

	pub fn get_thumbnails_path(&self) -> PathBuf
	{
		self.thumbnails_path.to_path_buf()
	}

	pub fn get_collections_path(&self) -> PathBuf
	{
		self.collections_path.to_path_buf()
	}
}

impl Filesystem
{
	pub(crate) fn insert(&self, object: &dyn ElementFilesystem) -> Result<(), Error>
	{
		object.insert_into(self)
	}
}
