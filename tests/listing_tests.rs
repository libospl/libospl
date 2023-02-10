mod test_tools;
use test_tools::generate_test_path;
use test_tools::remove_test_path;

#[cfg(test)]
mod tests
{
	use ospl::Library;
	
	#[test]
	fn list_all_photos()
	{
		let path = super::generate_test_path();
		let library = Library::create(&path).unwrap();

		let id_0 = library.import_photo("tests/files/test_photo_light.jpg").unwrap();
		let id_1 = library.import_photo("tests/files/test_photo_light.jpg").unwrap();
		
		let photos = library.list_all_photos().unwrap();
		assert_eq!(photos.get(0).unwrap().filename(), "test_photo_light.jpg");
		assert_eq!(photos.get(0).unwrap().id(), id_0);
		assert_eq!(photos.get(1).unwrap().id(), id_1);
		assert_eq!(photos.get(1).unwrap().filename(), "test_photo_light.jpg");
		super::remove_test_path(path);
	}

	#[test]
	fn list_all_thumbnails()
	{
		let path = super::generate_test_path();
		let library = Library::create(&path).unwrap();

		let id_0 = library.import_photo("tests/files/test_photo_light.jpg").unwrap();
		let id_1 = library.import_photo("tests/files/test_photo_light.jpg").unwrap();

		let photos = library.list_all_thumbnails().unwrap();
		assert_eq!(id_0, photos[0].0);
		assert_eq!(id_1, photos[1].0);
		for photo in photos
		{
			println!("photo id: {} | thumbnail_path: {:#?}", photo.0, photo.1);
		}
		super::remove_test_path(path);
	}

	#[test]
	fn list_all_collections()
	{
		let path = super::generate_test_path();
		let library = Library::create(&path).unwrap();

		let c0 = library.create_collection("test_collection_0", "").unwrap();
		let c1 = library.create_collection("test_collection_1", "").unwrap();

		let collections = library.list_all_collections().unwrap();
		assert_eq!(collections.get(0).unwrap().name(), "test_collection_0");
		assert_eq!(collections.get(0).unwrap().id(), c0.id());
		assert_eq!(collections.get(1).unwrap().id(), c1.id());
		assert_eq!(collections.get(1).unwrap().name(), "test_collection_1");
		super::remove_test_path(path);
	}

	#[test]
	fn list_albums_in_collection()
	{
		let path = super::generate_test_path();
		let library = Library::create(&path).unwrap();

		let c0 = library.create_collection("test_collection_0", "").unwrap();

		let a0 = library.create_album("test_album_0", "", c0.id()).unwrap();
		let a1 = library.create_album("test_album_1", "", c0.id()).unwrap();

		let albums = library.list_albums_in_collection(c0.id()).unwrap();
		assert_eq!(albums.get(0).unwrap().name(), "test_album_0");
		assert_eq!(albums.get(0).unwrap().id(), a0.id());
		assert_eq!(albums.get(1).unwrap().id(), a1.id());
		assert_eq!(albums.get(1).unwrap().name(), "test_album_1");
		super::remove_test_path(path);
	}

	#[test]
	fn list_photos_in_album()
	{
		let path = super::generate_test_path();
		let library = Library::create(&path).unwrap();

		let c0 = library.create_collection("test_collection_0", "").unwrap();
		let a0 = library.create_album("test_album_0", "", c0.id()).unwrap();

		let id_0 = library.import_photo("tests/files/test_photo_light.jpg").unwrap();
		let id_1 = library.import_photo("tests/files/test_photo_light.jpg").unwrap();

		library.assign_photo_to_album(id_0, a0.id()).unwrap();
		library.assign_photo_to_album(id_1, a0.id()).unwrap();

		let photos = library.list_photos_in_album(a0.id()).unwrap();
		assert_eq!(photos.get(0).unwrap().filename(), "test_photo_light.jpg");
		assert_eq!(photos.get(0).unwrap().id(), id_0);
		assert_eq!(photos.get(1).unwrap().id(), id_1);
		assert_eq!(photos.get(1).unwrap().filename(), "test_photo_light.jpg");
		super::remove_test_path(path);
	}
}
