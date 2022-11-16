pub struct Library
{
    database_path: String,
    db: sqlite::Connection,
}

pub fn new(db_path: String) -> Library {
    // Create empty database with correct structure.
    let connection = sqlite::open(db_path.clone()).unwrap();
    let query = "CREATE TABLE photos (path TEXT NOT NULL, age INTEGER);";
    connection.execute(query).unwrap();

    Library {database_path: db_path, db: connection}
}

pub fn open(db_path: String) -> Library {
    // Open.
    let connection = sqlite::open(db_path.clone()).unwrap();
    Library {database_path: db_path, db: connection}
}