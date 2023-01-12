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
use crate::Error;

use image::imageops::thumbnail;

// We should consider using a more efficient crate for creating thumbnails,
// such as libvips or imagemagick.
// Additional performance testing will be necessary to determine he optimal method.
// However, currently, this is not a top priority as the primary goal is to
// establish the basic functionality of the OSPL project.


#[allow(dead_code)]
pub fn create_thumbnail_from_path(photo_path: &str, save_to: &str) -> Result<(), Error>
{
	let img = image::open(std::path::Path::new(photo_path))?;
	let new_height: u32 = 325;
	let new_width: u32 = (img.width() * new_height) / img.height();
	let img = thumbnail(&img, new_width, new_height);
	img.save(std::path::Path::new(save_to))?;
	Ok(())
}
