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
use super::DATABASE_FILENAME;
use super::Error;

use rusqlite::{Connection};

static DATABASE_SQL: &str = include_str!("../database.sql");

pub struct Database
{
	pub path: String
}

impl Database
{
	pub(crate) fn create(path: &String) -> Result<Self, Error>
	{
		let connection = match Connection::open(path.clone() + "/" + DATABASE_FILENAME)
		{
			Ok(c) => Ok(c),
			Err(_why) => return Err(Error::Other)
		};
		
		match connection?.execute_batch(DATABASE_SQL)
		{
			Ok(_) =>
			{
				return Ok(Database
				{
					path: path.clone() + DATABASE_FILENAME,
				})
			},
			Err(_why) => return Err(Error::Other)
		}
	}

	pub(crate) fn create_collection(self, name: &String, comment: &String) -> Result<bool, Error>
	{
		/* Create a connection to the database. */
		let connection: Result<Connection, Error> = match Connection::open(self.path.clone())
		{
			Ok(c) => Ok(c),
			Err(_error) => return Err(Error::Other)
		};
		
		/* Create a table with the name and comment passed along. */
		match connection?.execute("INSERT INTO collections (name, comment) VALUES (?1, ?2)", (&name, &comment))
		{
			Ok(_result) => (),
    		Err(err) => {println!("Database collection creation failed: {}", err)},
		}
			
		Ok(true)
	}
}
