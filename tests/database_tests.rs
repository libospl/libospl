mod test_tools;
use test_tools::generate_test_path;
use test_tools::remove_test_path;

#[cfg(test)]
mod tests
{
	use ospl::Library;
	use ospl::Error;

	#[test]
	fn import_and_get()
	{
		let path = super::generate_test_path();
		let library = Library::create(&path).unwrap();

		match library.import_photo("tests/files/test_photo_light.jpg")
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
		super::remove_test_path(path);
	}

	#[test]
	fn get_id_not_exist()
	{
		let path = super::generate_test_path();
		let library = Library::create(&path).unwrap();

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
		super::remove_test_path(path);
	}

	#[test]
	fn delete_imported_photo()
	{
		let path = super::generate_test_path();
		let library = Library::create(&path).unwrap();

		library.import_photo("tests/files/test_photo_light.jpg").unwrap();
		library.delete_photo_by_id(1).unwrap();
		super::remove_test_path(path);
	}

	#[test]
	fn delete_not_imported_photo()
	{
		let path = super::generate_test_path();
		let library = Library::create(&path).unwrap();

		match library.delete_photo_by_id(1)
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

}
