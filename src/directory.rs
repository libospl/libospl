/*	libospl - Open Source Photo Library
	an opensource and multiplateform photo library management that can be used
	to store and sort all your photos.
	Copyright (C) 2019-2022 Angelo Frangione

	This program is free software; you can redistribute it and/or modify
	it under the terms of the GNU General Public License as published by
	the Free Software Foundation; either version 2 of the License, or
	(at your option) any later version.

	This program is distributed in the hope that it will be useful,
	but WITHOUT ANY WARRANTY; without even the implied warranty of
	MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
	GNU General Public License for more details.

	You should have received a copy of the GNU General Public License along
	with this program; if not, write to the Free Software Foundation, Inc.,
	51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
*/

use super::Error;
use crate::utility;

use std::fs;
use std::path::{Path, PathBuf};

#[cfg(all(unix))]
use std::os::unix::fs::PermissionsExt;

pub(crate) struct Directory
{
	path: PathBuf
}

impl Directory
{
	pub(crate) fn from<P: AsRef<Path>>(path: P) -> Result<Self, Error>
	{
		Ok(Directory 
		{
			path: path.as_ref().to_path_buf()
		})
	}

	pub(crate) fn create(self) -> Result<(), Error>
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
			Err(why) => Err(utility::match_io_errorkind(why.kind()))
		}
	}
}
