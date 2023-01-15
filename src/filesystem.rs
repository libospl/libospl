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
// use crate::element::ElementFilesystem;

pub struct Filesystem
{
	pub path: String,
}

impl Filesystem
{
	/// Creates a filesystem object, and returns it
	pub(crate) fn new(path: &str) -> Result<Self, Error>
	{
		return Ok(Filesystem
			{
				path: path.to_owned() + "/",
			});
	}

	/// Create the filesystem object and creates the main fs structure
	pub(crate) fn create(path: &str) -> Result<Self, Error>
	{
		let fs = Self::new(path)?;
		let thumbnails_path: String = fs.path.to_owned() + "/thumbnails";
		let pictures_path: String = fs.path.to_owned() + "/pictures";
		let collections_path: String = fs.path.to_owned() + "/collections";

		Directory::from(&thumbnails_path)?.create()?;
		Directory::from(&pictures_path)?.create()?;
		Directory::from(&collections_path)?.create()?;
		Ok(fs)
	}
}

impl Filesystem
{

}
