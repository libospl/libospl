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
use crate::element::traits::ElementListing;
use crate::Database;
use crate::Filesystem;
use crate::{Error, OsplError};

use exif::Exif;
use exif::{In, Tag};

use chrono::naive::NaiveDateTime;
use xxhash_rust::xxh3::xxh3_128;

use std::path::{Path, PathBuf};

/// Structure containing exif data
#[derive(Debug, PartialEq, Clone)]
pub struct ExifData
{
	pub make:			Option<String>,
	pub model:			Option<String>,
	pub lens:			Option<String>,
	pub aperture:		Option<f32>,
	pub focal_length:	Option<f32>,
	pub exposure_time:	Option<String>,
	pub exposure_mode:	Option<u16>,
	pub sensitivity:	Option<i32>,
	pub flash:			Option<bool>,
	pub title:			Option<String>,
	pub latitude:		Option<f64>,
	pub longitude:		Option<f64>,
}

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

	// Exifs
	make:				Option<String>,
	model:				Option<String>,
	lens:				Option<String>,
	aperture:			Option<f32>,
	focal_length:		Option<f32>,
	exposure_time:		Option<String>,
	exposure_mode:		Option<u16>,
	sensitivity:		Option<i32>,
	flash:				Option<bool>,
	title:				Option<String>,
	latitude:			Option<f64>,
	longitude:			Option<f64>,
	//Exif blob
	exif_data:			Option<Vec<u8>>,

	path_on_fs:			PathBuf,
}

impl Default for Photo
{
	fn default() -> Self
	{
		Photo::new()
	}
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

			make:				None,
			model:				None,
			lens:				None,
			aperture:			None,
			focal_length:		None,
			exposure_time:		None,
			exposure_mode:		None,
			sensitivity:		None,
			flash:				None,
			title:				None,
			latitude:			None,
			longitude:			None,
			exif_data:			None,

			path_on_fs:			Path::new("").to_path_buf(),
		}
	}

	fn import_exif(&mut self, exif: Exif)
	{
		self.exif_data = Some(exif.buf().into());

		let make = exif.get_field(Tag::Make, In::PRIMARY);

		if let Some(make) = make
		{
			self.make = Some(make.display_value().to_string().replace("\"", ""));
		}
		println!("imported exif: make: {:?}", self.make);

		let model = exif.get_field(Tag::Model, In::PRIMARY);
		if let Some(model) = model
		{
			self.model = Some(model.display_value().to_string().replace("\"", ""));
		}
		println!("imported exif: model: {:?}", self.model);

		let lens = exif.get_field(Tag::LensModel, In::PRIMARY);
		if let Some(lens) = lens
		{
			self.lens = Some(lens.display_value().to_string().replace("\"", ""));
		}
		println!("imported exif: lens: {:?}", self.lens);

		let aperture = exif.get_field(Tag::ApertureValue, In::PRIMARY);
		if let Some(aperture) = aperture
		{
			self.aperture = aperture.display_value().to_string().parse().ok();
		}
		println!("imported exif: aperture: {:?}", self.aperture);

		let focal_length = exif.get_field(Tag::FocalLength, In::PRIMARY);
		if let Some(focal_length) = focal_length
		{
			self.focal_length = focal_length.display_value().to_string().parse().ok();
		}
		println!("imported exif: focal_length: {:?}", self.focal_length);

		let exposure_time = exif.get_field(Tag::ExposureTime, In::PRIMARY);
		if let Some(exposure_time) = exposure_time
		{
			self.exposure_time = Some(exposure_time.display_value().to_string().replace("\"", ""));
		}
		println!("imported exif: exposure_time: {:?}", self.exposure_time);

		let exposure_mode = exif.get_field(Tag::ExposureMode, In::PRIMARY);
		if let Some(exposure_mode) = exposure_mode
		{
			self.exposure_mode = match exposure_mode.value.clone() {
				exif::Value::Short(v) => v.first().copied(),
				_ => None,
			};
		}
		println!("imported exif: exposure_mode: {:?}", self.exposure_mode);

		let sensitivity = exif.get_field(Tag::ISOSpeed, In::PRIMARY);
		if let Some(sensitivity) = sensitivity
		{
			self.sensitivity = sensitivity.display_value().to_string().parse().ok();
		}
		println!("imported exif: sensitivity: {:?}", self.sensitivity);

		let flash = exif.get_field(Tag::Flash, In::PRIMARY);
		if let Some(flash) = flash
		{
			self.flash = match flash.value.clone() {
				exif::Value::Short(v) => {
					match v.first() {
						Some(v) => Some((v & 1) != 0),
						None => None,
					}
				},
				_ => None,
			};
		}
		println!("imported exif: flash: {:?}", self.flash);

		let title = exif.get_field(Tag::ImageDescription, In::PRIMARY);
		if let Some(title) = title
		{
			self.title = Some(title.display_value().to_string().replace("\"", ""));
		}
		println!("imported exif: title: {:?}", self.title);

		let latitude = exif.get_field(Tag::GPSLatitude, In::PRIMARY);
		if let Some(latitude) = latitude
		{
			self.latitude = Some(latitude.display_value().to_string().parse().unwrap());
		}
		println!("imported exif: latitude: {:?}", self.latitude);

		let longitude = exif.get_field(Tag::GPSLongitude, In::PRIMARY);
		if let Some(longitude) = longitude
		{
			self.longitude = Some(longitude.display_value().to_string().parse().unwrap());
		}
		println!("imported exif: longitude: {:?}", self.longitude);
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
		let file = std::fs::read(&photo_path)?;
		self.hash = xxh3_128(&file);

		let file = std::fs::File::open(&photo_path)?;
		let mut bufreader = std::io::BufReader::new(&file);
		let exifreader = exif::Reader::new();
		let exif = exifreader.read_from_container(&mut bufreader);

		if let Ok(exif) = exif
		{
			self.import_exif(exif);
		}

		self.import_datetime = Some(chrono::offset::Local::now().naive_local());
		self.path_on_fs = photo_path.as_ref().to_path_buf();
		println!("import from file:\n{:#?}", &self.filename);
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

	pub fn get_exifs(&self) -> ExifData {
		ExifData {
			make: self.make.clone(),
			model: self.model.clone(),
			lens: self.lens.clone(),
			aperture: self.aperture,
			focal_length: self.focal_length,
			exposure_time: self.exposure_time.clone(),
			exposure_mode: self.exposure_mode,
			sensitivity: self.sensitivity,
			flash: self.flash,
			title: self.title.clone(),
			latitude: self.latitude,
			longitude: self.longitude,
		}
	}

	pub fn get_raw_exif(&self) -> Option<Vec<u8>> {
		self.exif_data.clone()
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
		db.connection.execute("DELETE FROM photos WHERE id = ?1", [&self.id])?;
		Ok(())
	}

	/// Insert a photo into the database, returns the id of it.
	fn insert_into(&self, db: &Database) -> Result<u32, OsplError>
	{
		db.connection.execute("INSERT INTO photos (filename, hash, import_datetime, make, model, lens, aperture, focal_length, exposure_time, exposure_mode, sensitivity, flash, title, latitude, longitude, exif_data ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16)",
		(&self.filename, &self.hash.to_ne_bytes(), &self.import_datetime, &self.make, &self.model, &self.lens, &self.aperture, &self.focal_length, &self.exposure_time, &self.exposure_mode, &self.sensitivity, &self.flash, &self.title, &self.latitude, &self.longitude, &self.exif_data))?;
		Ok(db.connection.last_insert_rowid() as u32)
	}

	#[cfg(not(tarpaulin_include))]
	fn rename(&self, _db: &Database, _new_name: &str) -> Result<(), OsplError>
	{
		unimplemented!()
	}

	/// loads the photo object with data from db with its id
	fn load_from_id(&mut self, db: &Database, id: u32) -> Result<(), OsplError>
	{
		let mut stmt = db.connection.prepare("SELECT * FROM photos WHERE id = ?1")?;
		let mut rows = stmt.query([&id])?;
		while let Some(row) = rows.next()?
		{
			self.id = row.get(0)?;
			self.filename = row.get(1)?;
			self.hash = u128::from_ne_bytes(row.get(2)?);
			self.import_datetime = row.get(4)?;

			self.rating = row.get(10)?;
			self.starred = row.get(11)?;
			self.make = row.get(12)?;
			self.model = row.get(13)?;
			self.lens = row.get(14)?;
			self.aperture = row.get(15)?;
			self.focal_length = row.get(16)?;
			self.exposure_time = row.get(17)?;
			self.exposure_mode = row.get(18)?;
			self.sensitivity = row.get(19)?;
			self.flash = row.get(20)?;
			self.title = row.get(21)?;
			self.latitude = row.get(22)?;
			self.longitude = row.get(23)?;
			self.exif_data = row.get(24)?;
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

				make:				row.get(12).ok(),
				model:				row.get(13).ok(),
				lens:				row.get(14).ok(),
				aperture:			row.get(15).ok(),
				focal_length:		row.get(16).ok(),
				exposure_time:		row.get(17).ok(),
				exposure_mode:		row.get(18).ok(),
				sensitivity:		row.get(19).ok(),
				flash:				row.get(20).ok(),
				title:				row.get(22).ok(),
				latitude:			row.get(23).ok(),
				longitude:			row.get(24).ok(),
				exif_data:			row.get(25).ok(),

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

				make:				None,
				model:				None,
				lens:				None,
				aperture:			None,
				focal_length:		None,
				exposure_time:		None,
				exposure_mode:		None,
				sensitivity:		None,
				flash:				None,
				title:				None,
				latitude:			None,
				longitude:			None,
				exif_data:			None,

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
