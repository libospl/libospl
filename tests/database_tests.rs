
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

	static TEST_DIR: &str = "/tmp/";

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
	fn import_and_get()
	{
		let path = generate_test_path();
		let library = Library::create(&path).unwrap();

		library.init().unwrap();

		match library.import_photo("tests/files/test_photo.jpg")
		{
			Ok(id) =>
			{
				println!("import id: {}", id);
				if id != 1
				{
					panic!("error: wrong id, first photo imported should be id 1");
				}
				println!("photo: {:#?}", library.get_photo_from_id(1));
				if library.get_photo_from_id(1).unwrap().id != 1
				{
					panic!("error: getting photo with id 1 but not reflected in result")
				}
			},
			Err(e) => panic!("error: importing not possible: {:?}", e)
		}
		remove_test_path(path);
	}

	#[test]
	fn get_id_not_exist()
	{
		let path = generate_test_path();
		let library = Library::create(&path).unwrap();

		library.init().unwrap();

		println!("{:#?}", library.get_photo_from_id(10));
		match library.get_photo_from_id(1)
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
}
