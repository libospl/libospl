mod test_tools;
use test_tools::generate_test_path;
use test_tools::remove_test_path;

#[cfg(test)]
mod tests
{
	use ospl::Library;
	use ospl::*;
	use rusqlite::{Connection};

	static TEST_DIR: &str = env!("CARGO_TARGET_TMPDIR");
	static LIBRARY_CREATE_ERROR: &str = "error creating library";

	fn check_table_presence<P: AsRef<std::path::Path>>(name: &str, db: P) -> bool
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
		let path = super::generate_test_path();
		println!("creating library at {:?}", path);
		let _library = match Library::create(&path)
		{
			Ok(lib) =>
			{
				println!("check if {:?} == {:?}", lib.get_path(), path);
				assert_eq!(lib.get_path(), path);
			},
			Err(e) => {panic!("{}: {:?}", LIBRARY_CREATE_ERROR, e)},
		};
		super::remove_test_path(path);
	}
	#[test]
	fn library_database()
	{
		let path = super::generate_test_path();
		let db_path = path.join("database.db");

		let _library = match Library::create(&path)
		{
			Ok(_lib) =>
			{
				println!("checking if database has been created at {:?}", &db_path);
				assert!(std::path::Path::new(&db_path).exists());
			},
			Err(e) => panic!("{}: {:?}", LIBRARY_CREATE_ERROR, e),
		};
		assert!(check_table_presence("settings", &db_path.to_str().unwrap()));
		assert!(check_table_presence("photos", &db_path));
		assert!(check_table_presence("collections", &db_path));
		assert!(check_table_presence("tags", &db_path));
		assert!(check_table_presence("photos_tags_map", &db_path));
		assert!(check_table_presence("albums", &db_path));
		super::remove_test_path(path);
	}

	#[test]
	#[cfg(target_os = "linux")]
	fn create_library_no_permissions()
	{
		assert_eq!(Library::create(&"/root/library".to_string()).err().unwrap(), Error::PermissionDenied);
	}

	#[test]
	fn create_library_exists()
	{
		assert_eq!(Library::create(&TEST_DIR.to_string()).err().unwrap(), Error::Exists);
	}

	#[test]
	fn library_init()
	{
		let path = super::generate_test_path();
		let _library = Library::create(&path).unwrap();

		assert!(std::path::Path::new(&(path.join("thumbnails"))).exists());
		assert!(std::path::Path::new(&(path.join("pictures"))).exists());
		assert!(std::path::Path::new(&(path.join("collections"))).exists());
		super::remove_test_path(path);
	}

	#[test]
	fn library_load()
	{
		let path = super::generate_test_path();
		let first_library = Library::create(&path).unwrap();
		first_library.import_photo("tests/files/test_photo_light.jpg").unwrap();
		let loaded_library = Library::load(&path).unwrap();

		assert_eq!(first_library.get_path(), loaded_library.get_path());
		loaded_library.get_photo_from_id(1).unwrap();
		super::remove_test_path(path);
	}
}
