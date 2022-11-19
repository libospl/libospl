static DATABASE_FILENAME: &str = "database.db";
static LIBRARY_EXTENSION: &str = ".ospl";

static VERSION_MAJOR: &str = env!("CARGO_PKG_VERSION_MAJOR");
static VERSION_MINOR: &str = env!("CARGO_PKG_VERSION_MINOR");
static VERSION_REVISION: &str = env!("CARGO_PKG_VERSION_PATCH");

mod database;
mod directory;

use database::Database;
use directory::Directory;

#[derive(Debug)]
pub enum Error
{
	Other = -1000,	// other error
	Exists,			// the file or folder already exists
	DB,				// database communication failed
	NotFound,		// element not found
	NotSupported,	// element not supported
	Thumb,			// thumbnail creation failed
	PhoNF,			// photo not found in db
	AlbNF,			// album not found in db
}

pub struct Library
{
	path: String,
	db: Database,
}

impl Library
{
	pub fn create(path: &String) -> Option<Self>
	{
		match Directory::from(&path).create()
		{
			Ok(_) =>
			{
				Some(Library
				{
					path: path.clone(),
					db: Database::create(&path),
				})
			},
			Err(n) => {println!("{:?}", n); None},
		}
	}
}

#[cfg(test)]
mod tests
{
	use super::*;

	use rand::{thread_rng, Rng};
	use rand::distributions::Alphanumeric;

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

	#[test]
	fn library_path()
	{
		let path = generate_test_path();
		let _library = match Library::create(&path)
		{
			Some(lib) =>
			{
				println!("checking if {} == {}", lib.path, path);
				assert_eq!(lib.path, path);
			},
			None => {panic!("{}", LIBRARY_CREATE_ERROR)},
		};
		remove_test_path(path);
	}
}
