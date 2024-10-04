mod test_tools;
use test_tools::generate_test_path;
use test_tools::remove_test_path;

#[cfg(test)]
mod exif_tests
{
	use ospl::Library;
	use ospl::element::photo::ExifData;

	#[test]
	fn import_eos_photo_and_check_all_exifs()
	{
		let path = super::generate_test_path();
		let library = Library::create(&path).unwrap();

		match library.import_photo("tests/files/test_photo_light.jpg")
		{
			Ok(_) => {},
			Err(e) => panic!("error: importing not possible: {:?}", e)
		}
		let p = library.get_photo_from_id(1).unwrap();
		println!("imported photo: {:?}", p.get_exifs());

		assert_eq!(p.get_exifs(), ExifData { make: Some("Canon".into()), model: Some("Canon EOS 70D".into()), lens: Some("EF50mm f/1.4 USM".into()), aperture: Some(2.375), focal_length: Some(50.0), exposure_time: Some("1/8000".into()), exposure_mode: Some(1), sensitivity: None, flash: Some(false), title: None, latitude: None, longitude: None });

		super::remove_test_path(path);
	}
}
