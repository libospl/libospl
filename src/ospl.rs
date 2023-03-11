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

mod thumbnails;

pub mod element;

pub mod api;

use std::io::ErrorKind;
use std::path::PathBuf;

use element::album::Album;
use database::Database;
use filesystem::Filesystem;
use element::photo::Photo;
use element::collection::Collection;

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
	/// Cannot be empty
	Empty,
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

impl Library // Get functions
{
	/// Get the path of where the library is located on disk
	///
	/// # Example
	/// ```no_run
	/// # use ospl::Library;
	/// let library = Library::create("/tmp/path.ospl/").unwrap();
	/// assert_eq!(library.get_path().to_str().unwrap(), "/tmp/path.ospl/");
	/// ```
	pub fn get_path(&self) -> PathBuf
	{
		self.fs.root_path()
	}
}
