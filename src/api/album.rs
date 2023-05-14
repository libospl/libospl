use crate::Library;
use crate::OsplError;
use crate::Error;
use crate::Database;
use crate::Album;
use crate::Photo;

impl Library
{
	/// Creates an album with provided name, comment and collection id
	///
	/// # Example
	/// ```no_run
	/// # use ospl::Library;
	/// let library = Library::load("/my/awesome/path.ospl/").unwrap();
	/// let collection_2030 = library.get_collection_from_id(35).unwrap();
	/// let album = library.create_album("Summer 2030", "All photos from my 2030 summer", collection_2030.id());
	/// ```
	pub fn create_album(&self, name: &str, comment: &str, collection: u32) -> Result<Album, OsplError>
	{
		let db = Database::new(self.fs.database_path())?;

		if name.is_empty()
		{
			return Err(OsplError::InternalError(Error::EmptyName));
		}
		let collection = self.get_collection_from_id(collection)?;
		let mut album = Album::new_with_name(name, comment, collection);
		let id = db.insert(&album)?;
		album.set_id(id);
		self.fs.insert(&album)?;
		Ok(album)
	}

	/// Get an Album element with its id
	///
	/// # Example
	/// ```no_run
	/// # use ospl::Library;
	/// let library = Library::load("/my/awesome/path.ospl/").unwrap();
	/// library.create_album("2019", "", 35);
	/// let album = library.get_album_from_id(23).unwrap();
	/// assert_eq!(album.id(), 23);
	/// assert_eq!(album.name(), "2019");
	/// assert_eq!(album.comment(), "");
	/// ```
	pub fn get_album_from_id(&self, id: u32) -> Result<Album, OsplError>
	{
		let db = Database::new(self.fs.database_path())?;
		let mut album = Album::new();
		db.from_id(&mut album, id)?;
		Ok(album)
	}

	/// Rename an album
	///
	/// # Example
	/// ```no_run
	/// # use ospl::Library;
	/// let library = Library::load("/my/awesome/path.ospl/").unwrap();
	/// let album = library.create_album("2019", "", 35).unwrap();
	/// assert_eq!(album.name(), "2019");
	/// library.rename_album_with_id(album.id(), "2020");
	/// let album = library.get_album_from_id(album.id()).unwrap();
	/// assert_eq!(album.name(), "2020");
	/// ```
	pub fn rename_album_with_id(&self, id: u32, new_name: &str) -> Result<(), OsplError>
	{
		let db = Database::new(self.fs.database_path())?;
		let album = self.get_album_from_id(id)?;
		self.fs.rename(&album, new_name)?;
		db.rename(&album, new_name)?;
		Ok(())
	}

	/// Move an album from the current collection to another one with it's id
	///
	/// # Example
	/// ```no_run
	/// # use ospl::Library;
	/// let library = Library::load("/my/awesome/path.ospl/").unwrap();
	/// let album = library.get_album_from_id(35).unwrap();
	/// let collection_2019 = library.get_collection_from_id(21).unwrap();
	/// let collection_2020 = library.get_collection_from_id(22).unwrap();
	/// assert_eq!(album.collection_id(), 21);
	/// library.move_album_by_id(album.id(), collection_2020.id());
	/// let album = library.get_album_from_id(35).unwrap();
	/// assert_eq!(album.collection_id(), 22);
	/// ```
	pub fn move_album_by_id(&self, album_id: u32, collection_id: u32) -> Result<(), OsplError>
	{
		let db = Database::new(self.fs.database_path())?;
		let album = self.get_album_from_id(album_id)?;
		let collection = self.get_collection_from_id(collection_id)?;
		album.move_to(&self.fs, &collection)?;
		album.assign_to(&db, &collection)?;
		Ok(())
	}

	/// Delete an album with its id
	///
	/// # Example
	/// ```no_run
	/// # use ospl::{Library, OsplError};
	/// let library = Library::load("/my/awesome/path.ospl/").unwrap();
	/// let album = library.get_album_from_id(35).unwrap();
	/// library.delete_album_by_id(35);
	/// assert_eq!(library.get_album_from_id(35).err().unwrap(), OsplError::IoError(std::io::ErrorKind::NotFound));
	/// ```
	pub fn delete_album_by_id(&self, id: u32) -> Result<(), OsplError>
	{
		let db = Database::new(self.fs.database_path())?;
		let album = self.get_album_from_id(id)?;
		self.fs.remove(&album)?;
		db.delete(&album)
	}

	/// Get all photos from an album in a Vec<Photo>
	///
	///	# Example
	/// ```no_run
	/// # use ospl::Library;
	/// let library = Library::load("/my/awesome/path.ospl/").unwrap();
	/// let photos = library.list_photos_in_album(1).unwrap();
	/// for photo in photos
	/// {
	/// 	println!("photo id: {} | name: {}", photo.id(), photo.get_filename());
	/// }
	/// ```
	pub fn list_photos_in_album(&self, album_id: u32) -> Result<Vec<Photo>, OsplError>
	{
		let db = Database::new(self.fs.database_path())?;
		<Album as crate::element::traits::InsideElementListing<Photo>>::list_inside(&db, album_id)
	}

	/// Assign a photo to an album
	///
	/// # Example
	/// ```no_run
	/// # use ospl::{Library, Error};
	/// let library = Library::load("/my/awesome/path.ospl/").unwrap();
	/// let album = library.get_album_from_id(35).unwrap();
	/// library.assign_photo_to_album(27, album.id());
	/// ```
	pub fn assign_photo_to_album(&self, photo: u32, album: u32) -> Result<(), OsplError>
	{
		let db = Database::new(self.fs.database_path())?;
		let album = self.get_album_from_id(album)?;
		let photo = self.get_photo_from_id(photo)?;
		album.put(&db, &photo)?;
		album.add(&self.fs, &photo)?;
		Ok(())
	}
}
