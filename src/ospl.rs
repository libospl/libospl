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

//! libopl is a library of function to help developper making good photo library management interfaces.
//!
//! ospl creates a database for you, parses all metadata and can support a lot of different things other
//! photo manager can do, but this crate is only meant to be used as an API.

pub static DATABASE_FILENAME: &str = "database.db";
pub static LIBRARY_EXTENSION: &str = ".ospl";

pub static VERSION_MAJOR: &str = env!("CARGO_PKG_VERSION_MAJOR");
pub static VERSION_MINOR: &str = env!("CARGO_PKG_VERSION_MINOR");
pub static VERSION_REVISION: &str = env!("CARGO_PKG_VERSION_PATCH");

mod database;
mod directory;

use database::Database;
use directory::Directory;

#[derive(Debug)]
pub enum Error
{
	/// other error
	Other = -1000,
	/// the file or folder already exists
	Exists,
	/// database communication failed
	DB,
	/// element not found
	NotFound,
	/// no permission to create or read file
	PermissionDenied,
	/// element not supported
	NotSupported,
	/// thumbnail creation failed
	Thumb,
	/// photo not found in db
	PhoNF,
	/// album not found in db
	AlbNF,
}

pub struct Library
{
	pub path: String,
	pub db: Database,
}

impl Library
{
	/// Creates a new empty ospl library.
	///
	/// This function will create a folder at the given path, create the database file
	/// and the subfolders the library needs to work.
	///
	/// # Example
	///
	/// ```
	/// # use ospl::Library;
	/// let library = match Library::create(&"/my/awesome/path.ospl/".to_string())
	/// {
	/// 	Ok(_) => println!("Success!"),
	/// 	Err(e) => println!("An error occured: {:?}", e),
	/// };
	///
	///
	pub fn create(path: &String) -> Result<Self, Error>
	{
		match Directory::from(&path)?.create()
		{
			Ok(_) =>
			{
				Ok(Library
				{
					path: path.clone(),
					db: Database::create(&path)?
				})
			},
			Err(e) => Err(e),
		}
	}

	/// Initializes the folders needed to import pictures, create collections and albums.
	///
	/// # Example
	///
	/// ```no_run
	/// # use ospl::Library;
	/// let library = Library::create(&"/my/awesome/path.ospl/".to_string()).unwrap();
	/// library.init().unwrap();
	///
	pub fn init(self) -> Result <(), Error>
	{
		let thumbnails_path: String = self.path.to_owned() + "/thumbnails";
		let pictures_path: String = self.path.to_owned() + "/pictures";
		let collections_path: String = self.path.to_owned() + "/collections";

		Directory::from(&thumbnails_path)?.create()?;
		Directory::from(&pictures_path)?.create()?;
		Directory::from(&collections_path)?.create()?;
		Ok(())
	}
}

