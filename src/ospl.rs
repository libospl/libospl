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
	pub fn create(path: &String) -> Self
	{
		let lib = Library
		{
			path: path.clone(),
			db: Database::create(&path),
		};
		match Directory::from(&path).create()
		{
			Ok(_) => {} ,
			Err(n) => println!("{:?}", n),
		}
		lib
	}
}

#[cfg(test)]
mod tests 
{
	use super::*;

	#[test]
	fn it_works()
	{
		let library = Library::create(&"~/Pictures/photos.ospl".to_string());
		assert_eq!(library.path, "~/Pictures/photos.ospl");
	}
}
