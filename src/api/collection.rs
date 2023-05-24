use crate::Library;
use crate::OsplError;
use crate::Database;
use crate::Collection;
use crate::Album;

impl Library
{
	/// Get all collections in a Vec<Collection>
	///
	/// # Example
	/// ```no_run
	/// # use ospl::Library;
	/// let library = Library::load("/my/awesome/path.ospl/").unwrap();
	/// let collections = library.list_all_collections().unwrap();
	/// for collection in collections
	/// {
	///     println!("collection id: {} | name: {}", collection.id(), collection.name());
	/// }
	/// ```
	pub fn list_all_collections(&self) -> Result<Vec<Collection>, OsplError>
	{
		let db = Database::new(self.fs.database_path())?;
		<Collection as crate::element::traits::ElementListing<Collection>>::list_all(&db, &self.fs)
	}


	/// Creates a collection
	///
	/// # Example
	/// ```no_run
	/// # use ospl::Library;
	/// let library = Library::create("/my/awesome/path.ospl/").unwrap();
	/// match library.create_collection("2019", "Photos from 2019") {
	///     Ok(_) => {},
	///     Err(err) => {panic!("Error creating collection: {:?}", err)}
	/// };
	///```
	pub fn create_collection(&self, name: &str, comment: &str) -> Result<Collection, OsplError>
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
	/// let library = Library::create("/my/awesome/path.ospl/").unwrap();
	/// library.create_collection("2019", "Photos from 2019").unwrap();
	/// let collection = library.get_collection_from_id(1).unwrap();
	/// assert_eq!("2019", collection.name());
	/// assert_eq!("Photos from 2019", collection.comment());
	/// assert_eq!(1, collection.id());
	///```
	pub fn get_collection_from_id(&self, id: u32) -> Result<Collection, OsplError>
	{
		let db = Database::new(self.fs.database_path())?;
		let mut collection = Collection::default();
		db.load_from_id(&mut collection, id)?;
		Ok(collection)
	}

	/// Rename a Collection element using its id
	///
	/// # Example
	/// ```no_run
	/// # use ospl::Library;
	/// let library = Library::create("/my/awesome/path.ospl/").unwrap();
	/// let collection = library.create_collection("Bird", "Contains my best bird pics").unwrap();
	/// assert_eq!("Bird", collection.name());
	/// library.rename_collection_with_id(collection.id(), "Birds").unwrap();
	/// let collection = library.get_collection_from_id(collection.id()).unwrap();
	/// assert_eq!("Birds", collection.name());
	/// ```
	pub fn rename_collection_with_id(&self, id: u32, new_name: &str) -> Result<(), OsplError>
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
	/// let library = Library::create("/my/awesome/path.ospl/").unwrap();
	/// let collection = library.create_collection("Vacations", "Best photos of my holidays").unwrap();
	/// library.delete_collection_by_id(collection.id());
	///```
	pub fn delete_collection_by_id(&self, id: u32) -> Result<(), OsplError>
	{
		let db = Database::new(self.fs.database_path())?;
		let collection = self.get_collection_from_id(id)?;
		self.fs.remove(&collection)?;
		db.delete(&collection)
	}

		/// Get all albums in a Vec<Album>
	///
	/// # Example
	/// ```no_run
	/// # use ospl::Library;
	/// let library = Library::load("/my/awesome/path.ospl/").unwrap();
	/// let albums = library.list_albums_in_collection(1).unwrap();
	/// for album in albums
	/// {
	///     println!("album id: {} | name: {}", album.id(), album.name());
	/// }
	/// ``` 
	pub fn list_albums_in_collection(&self, collection: u32) -> Result<Vec<Album>, OsplError>
	{
		let db = Database::new(self.fs.database_path())?;
		<Collection as crate::element::traits::InsideElementListing<Album>>::list_inside(&db, collection)
	}

}
