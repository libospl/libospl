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
mod filesystem;
mod directory;

mod thumbnails;

pub mod element;
pub mod photo;
pub mod collection;

use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use database::Database;
use filesystem::Filesystem;
use directory::Directory;
use photo::Photo;
use collection::Collection;

#[derive(Debug, PartialEq)]
pub enum Error
{
	/// other error
	Other = -1000,
	/// the file or folder already exists
	Exists,
	/// database communication failed
	DatabaseError,
	/// file or element not found
	NotFound,
	/// no permission to create or read file
	PermissionDenied,
	/// file or element not supported
	NotSupported,
	/// file is not an image file
	NotAnImage,
	/// A directory was specified when a non-directory was expected.
	IsADirectory,
	/// An error related to io
	IoError,
}

#[cfg(not(tarpaulin_include))]
impl From<rusqlite::Error> for Error
{
	fn from(error: rusqlite::Error) -> Self
	{
		match error
		{
			rusqlite::Error::SqliteFailure(error, _) => match error.code
			{
				_ => Error::DatabaseError,
			},
			rusqlite::Error::QueryReturnedNoRows => Error::DatabaseError,
			_ => Error::DatabaseError,
		}
	}
}

#[cfg(not(tarpaulin_include))]
impl From<std::io::Error> for Error
{
	fn from(error: std::io::Error) -> Self
	{
		match error.kind()
		{
			ErrorKind::AlreadyExists => Error::Exists,
			ErrorKind::PermissionDenied => Error::PermissionDenied,
			e =>
			{
				println!("error: {}", e);
				Error::IoError
			},
		}
	}
}

#[cfg(not(tarpaulin_include))]
impl From<image::ImageError> for Error
{
	fn from(error: image::ImageError) -> Self
	{
		match error
		{
			image::ImageError::Unsupported(_) => Error::NotSupported,
			image::ImageError::IoError(_) => Error::Other,
			image::ImageError::Decoding(_) => Error::NotAnImage,
			image::ImageError::Limits(_) => Error::Other,
			_ => Error::Other,
		}
	}
}
#[derive(Debug)]
pub struct Library
{
	fs: Filesystem,
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
	///```
	pub fn create<P: AsRef<Path>>(path: P) -> Result<Self, Error>
	{
		match Directory::from(&path)?.create()
		{
			Ok(_) =>
			{
				Ok(Library
				{
					fs: Filesystem::create(path)?,
				})
			},
			Err(e) => Err(e),
		}
	}

	/// Loads an existing ospl Library
	/// 
	/// # Example
	/// ```
	/// # use ospl::Library;
	/// let library = match Library::load(&"/my/awesome/path.ospl/".to_string())
	/// {
	/// 	Ok(_) => println!("Success!"),
	/// 	Err(e) => println!("An error occured: {:?}", e),
	/// };
	/// ```
	pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Error>
	{	
		Ok(Library
		{
			fs: Filesystem::new(path)?,
		})
	}

	/// Imports a photo into the photo library
	///
	/// # Example
	///
	/// ```no_run
	/// # use ospl::Library;
	/// let library = Library::create(&"/my/awesome/path.ospl/".to_string()).unwrap();
	/// library.import_photo("my_awesome_picture.jpg");
	///```
	pub fn import_photo<P: AsRef<Path>>(&self, photo_path: P) -> Result<u32, Error>
	{
		let db = Database::new(self.fs.database_path())?;
		if !photo_path.as_ref().exists()
		{
			return Err(Error::NotFound);
		}
		let mut photo = Photo::new();
		photo.from_file(&db, &photo_path)?;
		let id = db.insert(&photo)?;
		self.fs.insert(&photo)?;
		thumbnails::create_thumbnail_from_path(photo_path, self.fs.thumbnails_path().join(photo.get_filename()))?;
		Ok(id)
	}

	/// Get a Photo element from an id
	///
	/// # Example
	/// ```no_run
	/// # use ospl::Library;
	/// let library = Library::create(&"/my/awesome/path.ospl/".to_string()).unwrap();
	/// library.import_photo("my_awesome_picture.jpg");
	/// let photo = library.get_photo_from_id(1);
	/// println!("Photo: {:?}", photo);
	///```
	pub fn get_photo_from_id(&self, id: u32) -> Result<Photo, Error>
	{
		let db = Database::new(self.fs.database_path())?;
		let mut photo = Photo::new();
		db.from_id(&mut photo, id)?;
		Ok(photo)
	}

	/// Deletes a photo with given id
	///
	/// # Example
	/// ```no_run
	/// # use ospl::Library;
	/// let library = Library::create(&"/my/awesome/path.ospl/".to_string()).unwrap();
	/// library.import_photo("my_awesome_picture.jpg");
	/// library.delete_photo_by_id(1);
	///```
	pub fn delete_photo_by_id(&self, id: u32) -> Result<(), Error>
	{
		let db = Database::new(self.fs.database_path())?;
		let photo = self.get_photo_from_id(id)?;
		self.fs.remove(&photo)?;
		db.delete(&photo)
	}

	/// Creates a collection
	///
	/// # Example
	/// ```no_run
	/// # use ospl::Library;
	///	let library = Library::create(&"/my/awesome/path.ospl/".to_string()).unwrap();
	///	match library.create_collection("2019", "Photos from 2019") {
	///		Ok(_) => {},
	///		Err(err) => {panic!("Error creating collection: {:?}", err)}
	///	};
	///```
	pub fn create_collection(&self, name: &str, comment: &str) -> Result<Collection, Error>
	{
		let db = Database::new(self.fs.database_path())?;
		// TODO: Add checking to see if the collection has not been created.

		let mut collection = Collection::new_with_name(name, comment);

		let id = db.insert(&collection)?;
		collection.set_id(id);
		self.fs.insert(&collection)?;
		Ok(collection)
	}

	/// Get a Collection element from an id
	///
	/// # Example
	/// ```no_run
	/// # use ospl::Library;
	///	let library = Library::create(&"/my/awesome/path.ospl/".to_string()).unwrap();
	///	library.create_collection("2019", "Photos from 2019").unwrap();
	/// let collection = library.get_collection_from_id(1).unwrap();
	/// assert_eq!("2019", collection.name());
	/// assert_eq!("Photos from 2019", collection.comment());
	/// assert_eq!(1, collection.id());
	///```
	pub fn get_collection_from_id(&self, id: u32) -> Result<Collection, Error>
	{
		let db = Database::new(self.fs.database_path())?;
		let mut collection = Collection::new();
		db.from_id(&mut collection, id)?;
		Ok(collection)
	}

	/// Rename a Collection element using its id
	///
	/// # Example
	/// ```no_run
	/// # use ospl::Library;
	/// let library = Library::create(&"/my/awesome/path.ospl/".to_string()).unwrap();
	/// let collection = library.create_collection("Bird", "Contains my best bird pics").unwrap();
	/// assert_eq!("Bird", collection.name());
	/// library.rename_collection_with_id(collection.id(), "Birds").unwrap();
	/// let collection = library.get_collection_from_id(collection.id()).unwrap();
	/// assert_eq!("Birds", collection.name());
	/// ```
	pub fn rename_collection_with_id(&self, id: u32, new_name: &str) -> Result<(), Error>
	{
		let db = Database::new(self.fs.database_path())?;
		let collection = self.get_collection_from_id(id)?;
		self.fs.rename(&collection, new_name)?;
		db.rename(&collection, new_name)?;
		Ok(())
	}

	/// Deletes a collection with the given id
	///
	/// # Example
	/// ```no_run
	/// # use ospl::Library;
	/// let library = Library::create(&"/my/awesome/path.ospl/".to_string()).unwrap();
	/// let collection = library.create_collection("Vacations", "Best photos of my holidays").unwrap();
	/// library.delete_collection_by_id(collection.id());
	///```
	pub fn delete_collection_by_id(&self, id: u32) -> Result<(), Error>
	{
		let db = Database::new(self.fs.database_path())?;
		let collection = self.get_collection_from_id(id)?;
		self.fs.remove(&collection)?;
		db.delete(&collection)
	}
}

impl Library // Get functions
{
	/// Get the path of where the library is located on disk
	///
	/// # Example
	/// ```no_run
	/// # use ospl::Library;
	/// let library = Library::create(&"/tmp/path.ospl/").unwrap();
	/// assert_eq!(library.get_path().to_str().unwrap(), "/tmp/path.ospl/");
	/// ```
	pub fn get_path(&self) -> PathBuf
	{
		self.fs.root_path()
	}
}
