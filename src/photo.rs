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
use crate::Database;
use crate::Error;
use crate::utility;

use xxhash_rust::xxh3::xxh3_128;

use std::path::Path;

/// Structure containing a replica of sqlite data
#[derive(Debug)]
#[allow(dead_code)]
pub struct Photo
{
	id:					u32,
	filename:			String,
	hash:				u128,
	import_datetime:	String,
	rating:				u32,
	starred:			bool,
}

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
			import_datetime:	String::from(""),
			rating:				0,
			starred:			false,
		}
	}

	/// Gets data from an image file and fills self with basic data:
	/// - filename
	/// - hash
	pub fn from_file(&mut self, _db: &Database, photo_path: &str)
	-> Result <(), Error>
	{
		if Path::new(photo_path).is_dir()
		{
			return Err(Error::IsADirectory);
		}
		if !is_photo(photo_path)?
		{
			return Err(Error::NotAnImage);
		}
		self.filename = get_filename_from(photo_path);
		self.hash = xxh3_128(&std::fs::read(photo_path).unwrap());
		println!("import from file:\n{:#?}", &self);
		Ok(())
	}
}

impl ElementDatabase for Photo
{
	fn insert_into(&self, db: &Database) -> Result<u32, Error>
	{
		match db.connection.execute("INSERT INTO photos (filename, hash) VALUES (?1, ?2)",
		(&self.filename, &self.hash.to_ne_bytes()))
		{
			Ok(_) => Ok(1),
			Err(_) => return Err(Error::Other)
		}
	}

	fn from_id(&mut self, db: &Database, id: u32) -> Result<(), Error>
	{
		// fill self with the photo table from the database with the id
		let mut stmt = db.connection.prepare("SELECT * FROM photos WHERE id = ?1").unwrap();
		let mut rows = stmt.query(&[&id]).unwrap();
		while let Some(row) = rows.next().unwrap()
		{
			self.id = row.get(0).unwrap();
			self.filename = row.get(1).unwrap();
			self.hash = u128::from_ne_bytes(row.get(2).unwrap());
		}
		Ok(())
	}
}

/// Checks if the file is an image
fn is_photo(path: &str) -> Result<bool, Error>
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

fn get_filename_from(path: &str) -> String
{
	Path::new(path)
	.file_name()
	.unwrap()
	.to_str()
	.unwrap()
	.to_string()
}
