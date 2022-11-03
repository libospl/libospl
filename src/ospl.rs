pub struct Library
{
    path: String,
    db: String,
}

pub fn create_library(path: String) -> Library
{
    let db = path.clone() + "database.db";
    let lib = Library {path: path, db: db};
    lib

}

#[cfg(test)]
mod tests 
{
    use super::*;

    #[test]
    fn it_works()
    {
        let library = create_library("~/Pictures/photos.ospl".to_string());
        assert_eq!(library.path, "~/Pictures/photos.ospl");
    }
}
