use super::DATABASE_FILENAME;

pub struct Database
{
	pub path: String
}

impl Database
{
	pub fn create(path: &String) -> Self
	{
		Database
		{
			path: path.clone() + DATABASE_FILENAME,
		}
	}
}
