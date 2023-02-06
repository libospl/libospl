mod library_tests;
use library_tests::generate_test_path;
use library_tests::remove_test_path;

#[cfg(test)]
mod tests
{
	use ospl::Library;
	use ospl::Error;

	#[test]
	fn create_collection()
	{
		let path = super::generate_test_path();
		let library = Library::create(&path).unwrap();
		match library.create_collection("2019", "Photos from 2019") {
			Ok(collection) =>
			{
				assert_eq!(1, collection.id());
				assert_eq!("2019", collection.name());
				assert_eq!("Photos from 2019", collection.comment());
			},
			Err(err) => {panic!("Error creating collection: {:?}", err)}
		};
		super::remove_test_path(path);
	}

	#[test]
	fn get_id_not_exist()
	{
		let path = super::generate_test_path();
		let library = Library::create(&path).unwrap();

		match library.get_collection_from_id(1)
		{
			Err(e) =>
			{
				if e != Error::NotFound
				{
					panic!("error: an other error as NotFound is returned")
				}
			}
			Ok(_) => panic!("error: should not return Ok() with an unexisting id")
		}
		super::remove_test_path(path);
	}

	#[test]
	fn create_collection_test_values()
	{
		let path = super::generate_test_path();
		let library = Library::create(&path).unwrap();

		let name = "2019";
		let comment = "Photos from 2019";
		library.create_collection(name, comment).unwrap();
		let collection = library.get_collection_from_id(1).unwrap();
		assert!(std::path::Path::new(&library.get_path().join("collections").join(name)).exists());
		assert_eq!(1, collection.id());
		assert_eq!(name, collection.name());
		assert_eq!(comment, collection.comment());

		super::remove_test_path(path);
	}

	#[test]
	fn rename_collection()
	{
		let path = super::generate_test_path();
		let library = Library::create(&path).unwrap();
		let collection = library.create_collection("Bird", "Contains my best bird pics").unwrap();
		assert_eq!("Bird", collection.name());
		library.rename_collection_with_id(collection.id(), "Birds").unwrap();
		let collection = library.get_collection_from_id(collection.id()).unwrap();
		assert_eq!("Birds", collection.name());
		assert!(std::path::Path::new(&library.get_path().join("collections").join("Birds")).exists());
		assert!(!std::path::Path::new(&library.get_path().join("collections").join("Bird")).exists());
		super::remove_test_path(path);
	}

	#[test]
	fn remove_collection()
	{
		let path = super::generate_test_path();
		let library = Library::create(&path).unwrap();
		library.create_collection("name", "comment").unwrap();
		assert!(std::path::Path::new(&library.get_path().join("collections").join("name")).exists());
		library.delete_collection_by_id(1).unwrap();
		assert!(!std::path::Path::new(&library.get_path().join("collections").join("name")).exists());
		super::remove_test_path(path);
	}
}
