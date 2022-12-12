
#[cfg(test)]
mod tests
{
	use ospl::Library;
	use ospl::LIBRARY_EXTENSION;
	//use ospl::photo::Photo;

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
	fn import_single_photo()
	{
		let path = generate_test_path();
		let library = Library::create(&path).unwrap();

		library.init().unwrap();

		match library.import_photo("files/test_photo.jpg")
		{
			Ok(_) => {},
			Err(e) => panic!("error: importing not possible: {:?}", e)
		}
		remove_test_path(path);
	}
}

