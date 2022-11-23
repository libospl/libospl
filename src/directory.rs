use super::Error;

use std::fs;
use std::io::ErrorKind;

#[cfg(all(unix))]
use std::os::unix::fs::PermissionsExt;

pub struct Directory
{
	path: String
}

impl Directory
{
	pub fn from(path: &String) -> Result<Self, Error>
	{
		Ok(Directory 
		{
			path: path.clone() 
		})
	}

	pub fn create(self) -> Result<(), Error>
	{
		match fs::create_dir(&self.path)
		{
			Ok(_) =>
			{
				#[cfg(all(unix))]
				{
					match fs::set_permissions(&self.path, fs::Permissions::from_mode(0o777))
					{
						Ok(_) => {},
						Err(_) => {},
					}
				}
				Ok(())
			},
			Err(why) =>
			{
				match why.kind()
				{
					ErrorKind::AlreadyExists => return Err(Error::Exists),
					ErrorKind::PermissionDenied => return Err(Error::PermissionDenied),
					_ => return Err(Error::Other),
				}
			},
		}
	}
}
