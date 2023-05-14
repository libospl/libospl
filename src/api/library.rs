use crate::Library;
use crate::OsplError;
use crate::Filesystem;

use std::path::Path;

impl Library
{
	/// Creates a new empty ospl library.
	///
	/// This function will create a folder at the given path, create the database file
	/// and the subfolders the library needs to work.
	///
	/// # Example
	///
	/// ```
	/// # use ospl::Library;
	/// let library = match Library::create("/my/awesome/path.ospl/")
	/// {
	///     Ok(_) => println!("Success!"),
	///     Err(e) => println!("An error occured: {:?}", e),
	/// };
	///```
	pub fn create<P: AsRef<Path>>(path: P) -> Result<Self, OsplError>
	{
		std::fs::create_dir(&path)?;
		Ok(Library
		{
			fs: Filesystem::create(path)?,
		})
	}

	/// Loads an existing ospl Library
	///
	/// # Example
	/// ```
	/// # use ospl::Library;
	/// let library = match Library::load("/my/awesome/path.ospl/")
	/// {
	///     Ok(_) => println!("Success!"),
	///     Err(e) => println!("An error occured: {:?}", e),
	/// };
	/// ```
	pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, OsplError>
	{	
		Ok(Library
		{
			fs: Filesystem::new(path)?,
		})
	}
}
