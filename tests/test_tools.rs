static TEST_DIR: &str = env!("CARGO_TARGET_TMPDIR");

use std::path::{Path, PathBuf};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

use ospl::LIBRARY_EXTENSION;

pub fn generate_test_path() -> PathBuf
{
	let rand_string: String = thread_rng()
		.sample_iter(&Alphanumeric)
		.take(30)
		.map(char::from)
		.collect();
	Path::new(TEST_DIR).join(rand_string + LIBRARY_EXTENSION).to_path_buf()
}

pub fn remove_test_path<P: AsRef<Path>>(path: P)
{
	println!("removing test dir");
	match std::fs::remove_dir_all(path)
	{
		Ok(_) => {},
		Err(e) => {println!("{:?}", e)}
	}
}
