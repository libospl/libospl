mod library_tests;
use library_tests::generate_test_path;
use library_tests::remove_test_path;

#[cfg(test)]
mod tests
{
	use ospl::Library;
	use ospl::Error;

	#[cfg(target_os = "linux")]
	use std::fs;

	#[test]
	fn import_single_photo()
	{
		let path = super::generate_test_path();
		let library = Library::create(&path).unwrap();

		match library.import_photo("tests/files/test_photo_light.jpg")
		{
			Ok(_) => {},
			Err(e) => panic!("error: importing not possible: {:?}", e)
		}
		let p = library.get_photo_from_id(1);
		println!("imported photo: {:#?}", p);
		let filename = p.unwrap().get_filename();
		let photo_path = path.join("pictures").join(&filename);
		let thumb_path = path.join("thumbnails").join(&filename);
		println!("FULL_PATH: {:?}", photo_path);
		assert!(std::path::Path::new(&photo_path).exists());
		assert!(std::path::Path::new(&thumb_path).exists());
		super::remove_test_path(path);
	}

	#[test]
	fn import_single_photo_on_folder()
	{
		let path = super::generate_test_path();
		let library = Library::create(&path).unwrap();
		assert_eq!(library.import_photo("tests/files/test_folder/").err().unwrap(), Error::IsADirectory);
		super::remove_test_path(path);
	}

	#[test]
	fn import_photo_file_not_an_image()
	{
		let path = super::generate_test_path();
		let library = Library::create(&path).unwrap();
		assert_eq!(library.import_photo("tests/files/not_an_image.odt").err().unwrap(), Error::NotAnImage);
		super::remove_test_path(path);
	}

	#[test]
	fn import_photo_file_not_valid()
	{
		let path = super::generate_test_path();
		let library = Library::create(&path).unwrap();
		assert_eq!(library.import_photo("tests/files/not_a_valid_file.png").err().unwrap(), Error::NotAnImage);
		super::remove_test_path(path);
	}

	#[test]
	fn import_photo_no_string()
	{
		let path = super::generate_test_path();
		let library = Library::create(&path).unwrap();
		assert_eq!(library.import_photo("").err().unwrap(), Error::NotFound);
		super::remove_test_path(path);
	}

	#[test]
	#[cfg(target_os = "linux")]
	fn import_photo_permission_denied()
	{
		let path = super::generate_test_path();
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
		super::remove_test_path(path);
	}
}
