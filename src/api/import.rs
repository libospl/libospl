use crate::Library;
use crate::Database;
use crate::OsplError;
use crate::thumbnails;
use crate::Photo;

use std::path::Path;
	
impl Library
{
    /// Imports a photo into the photo library
	///
	/// # Example
	///
	/// ```no_run
	/// # use ospl::Library;
	/// let library = Library::create("/my/awesome/path.ospl/").unwrap();
	/// library.import_photo("my_awesome_picture.jpg");
	///```
	pub fn import_photo<P: AsRef<Path>>(&self, photo_path: P) -> Result<u32, OsplError>
	{
		let db = Database::new(self.fs.database_path())?;
		if !photo_path.as_ref().exists()
		{
			return Err(OsplError::IoError(std::io::ErrorKind::NotFound));
		}
		let mut photo = Photo::default();
		photo.from_file(&db, &photo_path)?;
		let id = db.insert(&photo)?;
		self.fs.insert(&photo)?;
		thumbnails::create_thumbnail_from_path(photo_path, self.fs.thumbnails_path().join(photo.get_filename()))?;
		Ok(id)
	}

	pub fn import_photo_into_album<P: AsRef<Path>>(&self, photo_path: P, album_id: u32) -> Result<u32, OsplError>
	{
		if !photo_path.as_ref().exists()
		{
			return Err(OsplError::IoError(std::io::ErrorKind::NotFound));
		}
		let photo = self.import_photo(photo_path)?;
		self.assign_photo_to_album(photo, album_id)?;
		Ok(photo)
	}

	pub fn import_folder_into_album<P: AsRef<Path>>(&self, folder_path: P, album_id: u32) -> Result<Vec<Result<u32, OsplError>>, OsplError>
	{
		if !folder_path.as_ref().exists()
		{
			return Err(OsplError::IoError(std::io::ErrorKind::NotFound));
		}
		let mut results: Vec<Result<u32, OsplError>> = Vec::new();
		println!("folder_path: {:?}", folder_path.as_ref());
		for entry in std::fs::read_dir(folder_path)?
		{
			println!("entry: {:?}", entry);
			let photo_path = entry?.path();
			if photo_path.is_file()
			{
				results.push(self.import_photo_into_album(photo_path, album_id));
			}
		}
		Ok(results)
	}
}
