extern crate chrono;
extern crate rand;
extern crate yui;

use std::error::Error;

use crate::core::{Lesson, StudentRecord};
use crate::spark::MainSpark;

mod core;
mod spark;

fn main() -> Result<(), Box<dyn Error>> {
	let student_record = StudentRecord::new(&LESSONS);
	yui::main(MainSpark { student_record })?;
	Ok(())
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