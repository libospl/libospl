use super::Error;
use super::DATABASE_FILENAME;

use rusqlite::Connection;

static DATABASE_SQL: &str = include_str!("../database.sql");

pub struct Database 
{
	pub path: String,
}

impl Database {
	pub fn create(path: &String) -> Result<Self, Error> 
	{
		let connection = match Connection::open(path.clone() + "/" + DATABASE_FILENAME) 
		{
			Ok(c) => Ok(c),
			Err(why) => return Err(Error::Other),
		};

		match connection?.execute_batch(DATABASE_SQL) 
		{
			Ok(_) => 
			{
				return Ok(Database 
				{
					path: path.clone() + DATABASE_FILENAME,
				})
			}
			Err(why) => return Err(Error::Other),
		}
	}
}
