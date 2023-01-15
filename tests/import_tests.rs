
#[cfg(test)]
mod tests
{
	use ospl::Library;
	use ospl::LIBRARY_EXTENSION;
	use ospl::Error;
	//use ospl::photo::Photo;

	use std::fs;
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

		match library.import_photo("tests/files/test_photo.jpg")
		{
			Ok(_) => {},
			Err(e) => panic!("error: importing not possible: {:?}", e)
		}
		remove_test_path(path);
	}

	#[test]
	fn import_single_photo_on_folder()
	{
		let path = generate_test_path();
		let library = Library::create(&path).unwrap();
		assert_eq!(library.import_photo("tests/files/test_folder/").err().unwrap(), Error::IsADirectory);
		remove_test_path(path);
	}

	#[test]
	fn import_photo_file_not_an_image()
	{
		let path = generate_test_path();
		let library = Library::create(&path).unwrap();
		assert_eq!(library.import_photo("tests/files/not_an_image.odt").err().unwrap(), Error::NotAnImage);
		remove_test_path(path);
	}

	#[test]
	fn import_photo_file_not_valid()
	{
		let path = generate_test_path();
		let library = Library::create(&path).unwrap();
		assert_eq!(library.import_photo("tests/files/not_a_valid_file.png").err().unwrap(), Error::NotSupported);
		remove_test_path(path);
	}

	#[test]
	fn import_photo_no_string()
	{
		let path = generate_test_path();
		let library = Library::create(&path).unwrap();
		assert_eq!(library.import_photo("").err().unwrap(), Error::NotFound);
		remove_test_path(path);
	}

	#[test]
	fn import_photo_permission_denied()
	{
		let path = generate_test_path();
		let library = Library::create(&path).unwrap();
		#[cfg(all(unix))]
		{
			use std::os::unix::fs::PermissionsExt;
			fs::set_permissions("tests/files/test_photo_no_permissions.jpg", fs::Permissions::from_mode(0o000)).unwrap();
		}
		assert_eq!(library.import_photo("tests/files/test_photo_no_permissions.jpg").err().unwrap(), Error::PermissionDenied);
		#[cfg(all(unix))]
		{
			let mut reset_perms = std::process::Command::new("chmod");
			reset_perms.arg("777").arg("tests/files/test_photo_no_permissions.jpg");
			reset_perms.status().expect("process failed to execute");
		}
		remove_test_path(path);
	}
}
