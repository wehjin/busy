extern crate chrono;
extern crate csv;
extern crate dirs;
extern crate echo_lib;
extern crate rand;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate yui;

use std::error::Error;
use std::path::PathBuf;

use echo_lib::kv;

pub use data::*;

use crate::constants::STORE_NAME;
use crate::spark::LaunchSpark;

mod data;
mod core;
mod spark;

#[cfg(debug_assertions)]
mod constants {
	pub const TAKE_COUNT: usize = 3;
	pub const STORE_NAME: &str = "kv-store-1-dbg";
}

#[cfg(not(debug_assertions))]
mod constants {
	pub const TAKE_COUNT: usize = 10;
	pub const STORE_NAME: &str = "kv-store-1";
}

fn main() -> Result<(), Box<dyn Error>> {
	let lessons = fetch_lessons()?;
	let data_folder = data_folder();
	let kv_store = kv::open(STORE_NAME, &data_folder)?;
	yui::main(LaunchSpark { lessons, kv_store })?;
	Ok(())
}

fn data_folder() -> PathBuf {
	let mut dir = dirs::home_dir().expect("No home directory");
	dir.push(".busy");
	dir
}
