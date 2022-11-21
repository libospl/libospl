use super::DATABASE_FILENAME;

use rusqlite::{Connection};

static DATABASE_SQL: &str = include_str!("../database.sql");

pub struct Database
{
	pub path: String
}

impl Database
{
	pub fn create(path: &String) -> Self
	{
		let connection = Connection::open(path.clone() + "/" + DATABASE_FILENAME).unwrap();
		match connection.execute_batch(DATABASE_SQL)
		{
			Ok(_) => println!("success!"),
			Err(e) => println!("error creating db: {:?}", e),
		};
		Database
		{
			path: path.clone() + DATABASE_FILENAME,
		}
	}
}
