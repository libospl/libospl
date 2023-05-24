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
}
