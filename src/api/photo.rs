use crate::Library;
use crate::Error;
use crate::Database;
use crate::Photo;

use std::path::PathBuf;

impl Library
{
	/// Get a Photo element from an id
	///
	/// # Example
	/// ```no_run
	/// # use ospl::Library;
	/// let library = Library::create("/my/awesome/path.ospl/").unwrap();
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

    /// Get all photos in a Vec, with only the id and the thumbnail path
	///
	/// This function is useful to show all photos consuming less memory
	/// because it only contains the id and the thumbnail path.
	/// To get more details about a photo call [Library::get_photo_from_id()]
	///
	/// # Example
	/// ```no_run
	/// # use ospl::Library;
	/// let library = Library::load("/my/awesome/path.ospl/").unwrap();
	/// let photos = library.list_all_thumbnails().unwrap();
	/// for photo in photos
	/// {
	/// 	println!("photo id: {} | thumbnail_path: {:#?}", photo.0, photo.1);
	/// }
	/// ```
	pub fn list_all_thumbnails(&self) -> Result<Vec<(u32, PathBuf)>, Error>
	{
		let db = Database::new(self.fs.database_path())?;
		<Photo as crate::element::traits::ElementListing<(u32, PathBuf)>>::list_all(&db, &self.fs)
	}

	/// Get all photos in a Vec<Photo>
	///
	/// This function gets all photos from the library, and all data related
	/// to the photos inside the Photo struct.
	/// To get all photos with less details use [Library::list_all_thumbnails()]
	///
	/// # Example
	/// ```no_run
	/// # use ospl::Library;
	/// let library = Library::load("/my/awesome/path.ospl/").unwrap();
	/// let photos = library.list_all_photos().unwrap();
	/// for photo in photos
	/// {
	/// 	println!("photo id: {} | thumbnail_path: {}", photo.id(), photo.get_filename());
	/// }
	/// ```
	pub fn list_all_photos(&self) -> Result<Vec<Photo>, Error>
	{
		let db = Database::new(self.fs.database_path())?;
		<Photo as crate::element::traits::ElementListing<Photo>>::list_all(&db, &self.fs)
	}



	/// Deletes a photo with given id
	///
	/// # Example
	/// ```no_run
	/// # use ospl::Library;
	/// let library = Library::create("/my/awesome/path.ospl/").unwrap();
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
}