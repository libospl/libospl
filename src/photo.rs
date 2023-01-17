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

use crate::element::ElementDatabase;
use crate::element::ElementFilesystem;
use crate::Database;
use crate::Filesystem;
use crate::Error;
use crate::utility;

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
	/// - hash
	pub fn from_file<P: AsRef<Path>>(&mut self, _db: &Database, photo_path: P)
	-> Result <(), Error>
	{
		if photo_path.as_ref().is_dir()
		{
			return Err(Error::IsADirectory);
		}
		if !is_photo(&photo_path)?
		{
			return Err(Error::NotAnImage);
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
	pub fn get_time_formatted(&self) -> String
	{
		std::format!("{}", self.import_datetime.unwrap().format("%Y-%m-%d_%H-%M-%S-%f"))
	}

	pub fn get_filename(&self) -> String
	{
		String::from(self.get_time_formatted() + "_" + &self.filename)
	}
}

impl ElementDatabase for Photo
{
	fn delete(&self, db: &Database) -> Result<(), Error>
	{
		match db.connection.execute("DELETE FROM photos WHERE id = ?1", &[&self.id])
		{
			Ok(_) => Ok(()),
			Err(_) => return Err(Error::Other)
		}
	}

	fn insert_into(&self, db: &Database) -> Result<u32, Error>
	{
		match db.connection.execute("INSERT INTO photos (filename, hash, import_datetime) VALUES (?1, ?2, ?3)",
		(&self.filename, &self.hash.to_ne_bytes(), &self.import_datetime))
		{
			Ok(_) => Ok(db.connection.last_insert_rowid() as u32),
			Err(_) => return Err(Error::Other)
		}
	}

	fn from_id(&mut self, db: &Database, id: u32) -> Result<(), Error>
	{
		// fill self with the photo table from the database with the id
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
			return Err(Error::NotFound);
		}
		Ok(())
	}
}

impl ElementFilesystem for Photo
{
	fn insert_into(&self, fs: &Filesystem) -> Result<(), Error>
	{
		let new_filename = self.get_time_formatted() + "_" + &self.filename;
		std::fs::copy(&self.path_on_fs, fs.get_pictures_path().join(new_filename))?;
		Ok(())
	}
}

/// Checks if the file is an image
fn is_photo<P: AsRef<Path>>(path: P) -> Result<bool, Error>
{
	let kind = match infer::get_from_path(path)
	{
		Ok(k) =>
		{
			match k // check the filetype
			{
				Some(ok) => ok.matcher_type(),
				None => return Err(Error::NotSupported),
			}
		}
		Err(e) => return Err(utility::match_io_errorkind(e.kind())),
	};
	if kind == infer::MatcherType::Image
	{
		return Ok(true);
	}
	Ok(false)
}

fn get_filename_from<P: AsRef<Path>>(path: P) -> String
{
	path.as_ref()
	.file_name()
	.unwrap()
	.to_str()
	.unwrap()
	.to_string()
}
