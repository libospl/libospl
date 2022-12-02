
#[cfg(test)]
mod tests
{
	use ospl::Library;
	use ospl::*;
	use ospl::LIBRARY_EXTENSION;
	
	use rand::{thread_rng, Rng};
	use rand::distributions::Alphanumeric;

	use rusqlite::{Connection};

	static TEST_DIR: &str = "/tmp/";
	static LIBRARY_CREATE_ERROR: &str = "error creating library";

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

	fn check_table_presence(name: &str, db: &str) -> bool
	{
		let conn = Connection::open(db).unwrap();
		let mut conn = conn.prepare(
			"SELECT name FROM sqlite_master WHERE type='table' AND name=?")
			.unwrap();
		let mut table: String = "".to_string();
		let mut rows = conn.query([name]).unwrap();
		while let Some(row) = rows.next().unwrap()
		{
			table = row.get(0).unwrap();
		}
		println!("check if table {} is present: table found: {}", name, table);
		name.eq(&table)
	}

	#[test]
	fn library_path()
	{
		let path = generate_test_path();
		let _library = match Library::create(&path)
		{
			Ok(lib) =>
			{
				println!("check if {} == {}", lib.path, path);
				assert_eq!(lib.path, path);
			},
			Err(e) => {panic!("{}: {:?}", LIBRARY_CREATE_ERROR, e)},
		};
		remove_test_path(path);
	}
	#[test]
	fn library_database()
	{
		let path = generate_test_path();
		let db_path = path.clone() + "/database.db";

		let _library = match Library::create(&path)
		{
			Ok(_lib) =>
			{
				println!("checking if database has been created at {}", &db_path);
				assert!(std::path::Path::new(&db_path).exists());
			},
			Err(e) => panic!("{}: {:?}", LIBRARY_CREATE_ERROR, e),
		};
		assert!(check_table_presence("settings", &db_path));
		assert!(check_table_presence("photos", &db_path));
		assert!(check_table_presence("collections", &db_path));
		assert!(check_table_presence("tags", &db_path));
		assert!(check_table_presence("photos_tags_map", &db_path));
		assert!(check_table_presence("albums", &db_path));
		remove_test_path(path);
	}

	#[test]
	#[should_panic]
	fn create_library_no_permissions()
	{
		let _library = match Library::create(&"/root/library".to_string())
		{
			Ok(_) => println!("trying to create library at path '/root/library' should return permission denied"),
			Err(e) =>
			{
				match e
				{
					Error::PermissionDenied => panic!("ok: could not create library at path '/root/library': permission denied"),
					_ => println!("error: unexpected error returned, but shouldn't have: {:?}", e),
				}
			}
		};
	}

	#[test]
	#[should_panic]
	fn create_library_exists()
	{
		let _library = match Library::create(&"/".to_string())
		{
			Ok(_) => println!("trying to create library at path '/' should return path already exists"),
			Err(e) =>
			{
				match e
				{
					Error::Exists => panic!("ok: could not create library at path '/': folder exists"),
					_ => println!("error: unexpected error returned, but shouldn't have: {:?}", e),
				}
			}
		};
	}

	#[test]
	fn library_init()
	{
		let path = generate_test_path();
		let library = Library::create(&path).unwrap();

		library.init().unwrap();
		assert!(std::path::Path::new(&(path.clone() + "/thumbnails")).exists());
		assert!(std::path::Path::new(&(path.clone() + "/pictures")).exists());
		assert!(std::path::Path::new(&(path.clone() + "/collections")).exists());
		remove_test_path(path);
	}
}
