
#[cfg(test)]
mod tests
{
	use ospl::Library;
	use ospl::LIBRARY_EXTENSION;
	use ospl::Error;
	//use ospl::photo::Photo;

	//use std::fs;
	use rand::{thread_rng, Rng};
	use rand::distributions::Alphanumeric;

	//use rusqlite::{Connection};

	static TEST_DIR: &str = env!("CARGO_TARGET_TMPDIR");

	fn remove_test_path(path: String)
	{
		println!("removing test dir");
		match std::fs::remove_dir_all(path)
		{
			Ok(_) => {},
			Err(e) => {println!("{:?}", e)}
		}
	}

	fn generate_test_path() -> String
	{
		let rand_string: String = thread_rng()
			.sample_iter(&Alphanumeric)
			.take(30)
			.map(char::from)
			.collect();
		TEST_DIR.to_string() + &rand_string + &LIBRARY_EXTENSION.to_string()
	}


	#[test]
	fn create_collection()
	{
		let path = generate_test_path();
		let library = Library::create(&path).unwrap();
		match library.create_collection("2019", "Photos from 2019") {
			Ok(_) => {},
			Err(err) => {panic!("Error creating collection: {:?}", err)}
		};
		remove_test_path(path);
	}

	#[test]
	fn get_id_not_exist()
	{
		let path = generate_test_path();
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
		remove_test_path(path);
	}

	#[test]
	fn create_collection_test_values()
	{
		let path = generate_test_path();
		let library = Library::create(&path).unwrap();

		let name = "2019";
		let comment = "Photos from 2019";
		let collection = library.create_collection(name, comment).unwrap();
		
		assert_eq!(name, collection.get_name());
		assert_eq!(comment, collection.get_comment());

		remove_test_path(path);
	}

}
