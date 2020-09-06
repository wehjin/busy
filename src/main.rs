extern crate chrono;
extern crate dirs;
extern crate echo_lib;
extern crate rand;
extern crate serde;
extern crate serde_json;
extern crate yui;

use std::error::Error;
use std::path::PathBuf;

use echo_lib::kv;

use crate::core::Lesson;
use crate::spark::LaunchSpark;

mod core;
mod spark;

fn main() -> Result<(), Box<dyn Error>> {
	let data_folder = data_folder();
	let kv_store = kv::open("kv-store-1", &data_folder)?;
	yui::main(LaunchSpark { lessons: &LESSONS, kv_store })?;
	Ok(())
}

fn data_folder() -> PathBuf {
	let mut dir = dirs::home_dir().expect("No home directory");
	dir.push(".busy");
	dir
}

const LESSONS: [Lesson; 19] = [
	Lesson::Recall(2, "embassy", "たいしかん"),
	Lesson::Recall(12, "kind,generous", "しんせつ"),
	Lesson::Recall(13, "vase", "かびん"),
	Lesson::Recall(13, "rose", "ばら"),
	Lesson::Recall(13, "bouquet", "はなたば"),
	Lesson::Recall(13, "ring", "ゆびわ"),
	Lesson::Recall(13, "anniversary", "きねんび"),
	Lesson::Recall(13, "suit, look good on", "にあいます"),
	Lesson::Recall(14, "clearance sale", "バーゲンセール"),
	Lesson::Recall(15, "ticket gate", "かいさつぐち"),
	Lesson::Recall(15, "fireworks display", "はなびたいかい"),
	Lesson::Recall(15, "large gathering", "たいかい"),
	Lesson::Recall(15, "snow festival", "ゆきまつり"),
	Lesson::Recall(15, "game, match", "しあい"),
	Lesson::Recall(15, "platform(train)", "ホーム"),
	Lesson::Recall(15, "circumstances", "つごう"),
	Lesson::Recall(15, "Asakusa Station", "あさくさえき"),
	Lesson::Recall(15, "Shimbashi Station", "しんばしえき"),
	Lesson::Recall(15, "hall(concert)", "ホール"),
];