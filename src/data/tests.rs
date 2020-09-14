use std::error::Error;

use super::{fetch_lessons, LESSONS};

#[test]
fn it_works() -> Result<(), Box<dyn Error>> {
	let lessons = fetch_lessons()?;
	assert!(lessons.len() > LESSONS.len());
	Ok(())
}
