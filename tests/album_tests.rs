mod test_tools;
use test_tools::generate_test_path;
use test_tools::remove_test_path;

#[cfg(test)]
mod tests
{
	use ospl::Library;
	use ospl::Error;

	#[test]
	fn create_album()
	{
		let path = super::generate_test_path();
		let library = Library::create(&path).unwrap();
		let collection = library.create_collection("2019", "Photos from 2019").unwrap();
		match library.create_album("Pizza Party", "My pizza party from 2019", collection.id())
		{
			Ok(album) =>
			{
				assert_eq!(1, album.id());
				assert_eq!("Pizza Party", album.name());
				assert_eq!("My pizza party from 2019", album.comment());
				assert!(std::path::Path::new(&library.get_path().join("collections").join("2019").join("Pizza Party")).exists());
			},
			Err(err) => {panic!("Error creating album: {:?}", err)}
		}
		super::remove_test_path(path);
	}

	#[test]
	fn get_album()
	{
		let path = super::generate_test_path();
		let library = Library::create(&path).unwrap();
		let collection = library.create_collection("2019", "Photos from 2019").unwrap();
		library.create_album("Pizza Party", "My pizza party from 2019", collection.id()).unwrap();
		let album = library.get_album_from_id(1).unwrap();
		assert_eq!(1, album.id());
		assert_eq!("Pizza Party", album.name());
		assert_eq!("My pizza party from 2019", album.comment());
		assert!(std::path::Path::new(&library.get_path().join("collections").join("2019").join("Pizza Party")).exists());
		super::remove_test_path(path);
	}

	#[test]
	fn rename_album()
	{
		let path = super::generate_test_path();
		let library = Library::create(&path).unwrap();
		let collection = library.create_collection("2019", "Photos from 2019").unwrap();
		let album = library.create_album("Pizza Party", "My pizza party from 2019", collection.id()).unwrap();
		library.rename_album_with_id(album.id(), "Pizza").unwrap();
		let album = library.get_album_from_id(album.id()).unwrap();
		assert_eq!(1, album.id());
		assert_eq!("Pizza", album.name());
		assert_eq!("My pizza party from 2019", album.comment());
		assert!(std::path::Path::new(&library.get_path().join("collections").join("2019").join("Pizza")).exists());
		super::remove_test_path(path);
	}

	#[test]
	fn get_unexisting_album()
	{
		let path = super::generate_test_path();
		let library = Library::create(&path).unwrap();
		assert_eq!(library.get_album_from_id(35).err().unwrap(), Error::NotFound);
		super::remove_test_path(path);
	}

	#[test]
	fn delete_album()
	{
		let path = super::generate_test_path();
		let library = Library::create(&path).unwrap();
		let collection = library.create_collection("2019", "Photos from 2019").unwrap();
		let album = library.create_album("Pizza", "My pizza party from 2019", collection.id()).unwrap();
		library.delete_album_by_id(album.id()).unwrap();
		assert!(!std::path::Path::new(&library.get_path().join("collections").join("2019").join("Pizza")).exists());
		assert_eq!(library.get_album_from_id(1).err().unwrap(), Error::NotFound);
		super::remove_test_path(path);
	}

	#[test]
	fn move_album()
	{
		let path = super::generate_test_path();
		let library = Library::create(&path).unwrap();
		let collection_2019 = library.create_collection("2019", "Photos from 2019").unwrap();
		let collection_2018 = library.create_collection("2018", "Photos from 2018").unwrap();
		let album = library.create_album("Pizza Party", "My pizza party from 2019", collection_2018.id()).unwrap();
		assert_eq!(album.collection_id(), collection_2018.id());
		assert!(std::path::Path::new(&library.get_path().join("collections").join("2018").join("Pizza Party")).exists());
		library.move_album_by_id(album.id(), collection_2019.id()).unwrap();
		let album = library.get_album_from_id(album.id()).unwrap();
		assert_eq!(album.collection_id(), collection_2019.id());
		assert!(std::path::Path::new(&library.get_path().join("collections").join("2019").join("Pizza Party")).exists());
		assert!(!std::path::Path::new(&library.get_path().join("collections").join("2018").join("Pizza Party")).exists());
		super::remove_test_path(path);
	}

	#[test]
	fn move_album_twice()
	{
		let path = super::generate_test_path();
		let library = Library::create(&path).unwrap();
		let collection_2019 = library.create_collection("2019", "Photos from 2019").unwrap();
		let collection_2018 = library.create_collection("2018", "Photos from 2018").unwrap();
		let album = library.create_album("Pizza Party", "My pizza party from 2019", collection_2018.id()).unwrap();
		assert_eq!(album.collection_id(), collection_2018.id());
		assert!(std::path::Path::new(&library.get_path().join("collections").join("2018").join("Pizza Party")).exists());
		library.move_album_by_id(album.id(), collection_2019.id()).unwrap();
		library.move_album_by_id(album.id(), collection_2019.id()).unwrap();
		let album = library.get_album_from_id(album.id()).unwrap();
		assert_eq!(album.collection_id(), collection_2019.id());
		assert!(std::path::Path::new(&library.get_path().join("collections").join("2019").join("Pizza Party")).exists());
		assert!(!std::path::Path::new(&library.get_path().join("collections").join("2018").join("Pizza Party")).exists());
		super::remove_test_path(path);
	}

	#[test]
	fn album_empty_comment()
	{
		let path = super::generate_test_path();
		let library = Library::create(&path).unwrap();
		let collection = library.create_collection("2019", "Photos from 2019").unwrap();
		let album = library.create_album("Pizza Party", "", collection.id()).unwrap();
		assert_eq!(album.comment(), "");
		super::remove_test_path(path);
	}
	#[test]
	fn album_empty_name()
	{
		let path = super::generate_test_path();
		let library = Library::create(&path).unwrap();
		let collection = library.create_collection("2019", "Photos from 2019").unwrap();
		let album = library.create_album("", "", collection.id());
		assert_eq!(album.err().unwrap(), Error::Empty);
		super::remove_test_path(path);
	}

	#[test]
	fn assign_photo_to_album()
	{
		let path = super::generate_test_path();
		let library = Library::create(&path).unwrap();
		let collection = library.create_collection("2019", "Photos from 2019").unwrap();
		let album = library.create_album("test", "", collection.id()).unwrap();
		let photo = 
		{
			let id = library.import_photo("tests/files/test_photo_light.jpg").unwrap();
			library.get_photo_from_id(id).unwrap()
		};
		library.assign_photo_to_album(photo.id(), album.id()).unwrap();
		assert!(std::path::Path::new(&library.get_path().join("collections").join("2019").join("test")).join(photo.get_filename()).exists());
		super::remove_test_path(path);
	}

#[test]
fn assign_photo_to_album_twice()
{
	let path = super::generate_test_path();
	let library = Library::create(&path).unwrap();
	let collection = library.create_collection("2019", "Photos from 2019").unwrap();
	let album = library.create_album("test", "", collection.id()).unwrap();
	let photo = 
	{
		let id = library.import_photo("tests/files/test_photo_light.jpg").unwrap();
		library.get_photo_from_id(id).unwrap()
	};
	library.assign_photo_to_album(photo.id(), album.id()).unwrap();
	library.assign_photo_to_album(photo.id(), album.id()).unwrap();
	assert!(std::path::Path::new(&library.get_path().join("collections").join("2019").join("test")).join(photo.get_filename()).exists());
	// super::remove_test_path(path);
	}

	#[test]
fn assign_photo_to_two_albums()
{
	let path = super::generate_test_path();
	let library = Library::create(&path).unwrap();
	let collection = library.create_collection("2019", "Photos from 2019").unwrap();
	let album = library.create_album("test", "", collection.id()).unwrap();
	let album_2 = library.create_album("test_2", "", collection.id()).unwrap();
	let photo = 
	{
		let id = library.import_photo("tests/files/test_photo_light.jpg").unwrap();
		library.get_photo_from_id(id).unwrap()
	};
	library.assign_photo_to_album(photo.id(), album.id()).unwrap();
	library.assign_photo_to_album(photo.id(), album_2.id()).unwrap();
	assert!(std::path::Path::new(&library.get_path().join("collections").join("2019").join("test")).join(photo.get_filename()).exists());
	assert!(std::path::Path::new(&library.get_path().join("collections").join("2019").join("test_2")).join(photo.get_filename()).exists());
	super::remove_test_path(path);
	}
}
