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

use crate::element::traits::ElementDatabase;
use crate::element::traits::ElementFilesystem;
use crate::Database;
use crate::Filesystem;
use crate::{Error, OsplError};
use crate::element::traits::ElementListing;

use chrono::naive::NaiveDateTime;
use xxhash_rust::xxh3::xxh3_128;

use std::path::{Path, PathBuf};

/// Structure containing a replica of sqlite data
#[derive(Debug)]
#[allow(dead_code)]
pub struct Photo
{
	pub id:				u32,
	filename:			String,
	hash:				u128,
	import_datetime:	Option<NaiveDateTime>,
	rating:				u32,
	starred:			bool,

	path_on_fs:			PathBuf,
}

// Constructors
impl Photo
{
	/// Returns an empty Photo element
	pub fn new() -> Self
	{
		Photo
		{
			id:					0,
			filename:			String::from(""),
			hash:				0,
			import_datetime:	None,
			rating:				0,
			starred:			false,

			path_on_fs:			Path::new("").to_path_buf(),
		}
	}

	/// Gets data from an image file and fills self with basic data:
	/// - filename
	/// - hash using xxh algorithm
	/// - import datetime
	/// - the current path on the filesystem
	pub fn from_file<P: AsRef<Path>>(&mut self, _db: &Database, photo_path: P)
	-> Result <(), OsplError>
	{
		if photo_path.as_ref().is_dir()
		{
			return Err(OsplError::InternalError(Error::IsADirectory));
		}
		if !is_photo(&photo_path)?
		{
			return Err(OsplError::InternalError(Error::NotAnImage));
		}
		self.filename = get_filename_from(&photo_path);
		self.hash = xxh3_128(&std::fs::read(&photo_path)?);
		self.import_datetime = Some(chrono::offset::Local::now().naive_local());
		self.path_on_fs = photo_path.as_ref().to_path_buf();
		println!("import from file:\n{:#?}", &self);
		Ok(())
	}
}

impl Photo
{
	pub fn id(&self) -> u32
	{
		self.id
	}

	pub fn filename(&self) -> String
	{
		self.filename.clone()
	}
}

impl Photo // Private function only useful to the local functions
{
	/// Get actual time formated specificly to be added in front of the photo filename
	fn get_time_formatted(&self) -> String
	{
		std::format!("{}", self.import_datetime.unwrap().format("%Y-%m-%d_%H-%M-%S-%f"))
	}
}
impl Photo // Public function to get information about the photo
{
	pub fn get_filename(&self) -> String
	{
		self.get_time_formatted() + "_" + &self.filename
	}
}

impl ElementDatabase for Photo
{
	/// Deletes the photo from the database with its id
	fn delete(&self, db: &Database) -> Result<(), OsplError>
	{
		db.connection.execute("DELETE FROM photos WHERE id = ?1", &[&self.id])?;
		Ok(())
	}

	/// Insert a photo into the database, returns the id of it.
	fn insert_into(&self, db: &Database) -> Result<u32, OsplError>
	{
		db.connection.execute("INSERT INTO photos (filename, hash, import_datetime) VALUES (?1, ?2, ?3)",
		(&self.filename, &self.hash.to_ne_bytes(), &self.import_datetime))?;
		Ok(db.connection.last_insert_rowid() as u32)
	}

	#[cfg(not(tarpaulin_include))]
	fn rename(&self, _db: &Database, _new_name: &str) -> Result<(), OsplError>
	{
		unimplemented!()
	}

	/// loads the photo object with data from db with its id
	fn from_id(&mut self, db: &Database, id: u32) -> Result<(), OsplError>
	{
		let mut stmt = db.connection.prepare("SELECT * FROM photos WHERE id = ?1")?;
		let mut rows = stmt.query(&[&id])?;
		while let Some(row) = rows.next()?
		{
			self.id = row.get(0)?;
			self.filename = row.get(1)?;
			self.hash = u128::from_ne_bytes(row.get(2)?);
			self.import_datetime = row.get(4)?;
		}
		if self.id == 0
		{
			return Err(OsplError::IoError(std::io::ErrorKind::NotFound));
		}
		Ok(())
	}
}

impl ElementFilesystem for Photo
{
	/// Inserts a photo into the filesystem using `self.path_on_fs` variable
	fn insert_into(&self, fs: &Filesystem) -> Result<(), OsplError>
	{
		std::fs::copy(&self.path_on_fs, fs.pictures_path().join(self.get_filename()))?;
		Ok(())
	}

	/// Remove everything related to a photo from the filesystem.
	///
	/// this includes the thumbnail, and in the future every reference to it in the albums
	fn remove_from(&self, fs: &Filesystem) -> Result<(), OsplError>
	{
		std::fs::remove_file(fs.pictures_path().join(self.get_filename()))?;
		std::fs::remove_file(fs.thumbnails_path().join(self.get_filename()))?;
		Ok(())
	}

	#[cfg(not(tarpaulin_include))]
	fn rename(&self, _fs: &Filesystem, _new_name: &str) -> Result<(), OsplError>
	{
		unimplemented!()
	}
}

/// Checks if the file is an image
fn is_photo<P: AsRef<Path>>(path: P) -> Result<bool, OsplError>
{
	match infer::get_from_path(path)?
	{
		Some(t) =>
		{
			if t.matcher_type() == infer::MatcherType::Image
			{
				return Ok(true);
			}
		}
		None =>	{ return Err(OsplError::InternalError(Error::NotAnImage)); }
	}
	Ok(false)
}

impl ElementListing<Photo> for Photo
{
	fn list_all(db: &Database, _fs: &Filesystem) -> Result<Vec<Photo>, OsplError>
	{
		let mut photos: Vec<Photo> = Vec::new();
		let mut stmt = db.connection.prepare("SELECT * FROM photos")?;
		let mut rows = stmt.query(())?;
		while let Some(row) = rows.next()?
		{
			let photo = Photo
			{
				id:					row.get(0)?,
				filename:			row.get(1)?,
				hash:				u128::from_ne_bytes(row.get(2)?),
				import_datetime:	row.get(4)?,
				rating:				row.get(10)?,
				starred:			row.get(11)?,

				path_on_fs:			Path::new("").to_path_buf(),
			};
			photos.push(photo);
		}
		Ok(photos)
	}
}
impl ElementListing<(u32, PathBuf)> for Photo
{
	fn list_all(db: &Database, fs: &Filesystem) -> Result<Vec<(u32, PathBuf)>, OsplError>
	{
		let mut photos: Vec<(u32, PathBuf)> = Vec::new();
		let mut stmt = db.connection.prepare("SELECT * FROM photos")?;
		let mut rows = stmt.query(())?;
		while let Some(row) = rows.next()?
		{
			let photo = Photo
			{
				id:					row.get(0)?,
				filename:			row.get(1)?,
				hash:				u128::from_ne_bytes(row.get(2)?),
				import_datetime:	row.get(4)?,
				rating:				row.get(10)?,
				starred:			row.get(11)?,

				path_on_fs:			Path::new("").to_path_buf(),
			};
			let thumbnail_path = fs.thumbnails_path().join(photo.get_filename());
			photos.push((photo.id(), thumbnail_path));
		}
		Ok(photos)
	}
}

/// Returns only the filename from a path
fn get_filename_from<P: AsRef<Path>>(path: P) -> String
{
	path.as_ref()
	.file_name()
	.unwrap()
	.to_str()
	.unwrap()
	.to_string()
}
