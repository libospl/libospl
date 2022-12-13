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

use std::io::ErrorKind;
use crate::Error;

pub fn match_io_errorkind(e: ErrorKind) -> Error
{
	match e
	{
		ErrorKind::AlreadyExists => return Error::Exists,
		ErrorKind::PermissionDenied => return Error::PermissionDenied,
		error => 
		{
			if cfg!(debug_assertion)
			{
				println!("{:?}", error);
			}
			return Error::Other;
		}
	}
}
