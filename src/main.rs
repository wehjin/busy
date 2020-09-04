extern crate chrono;
extern crate rand;

use std::io::{stdout, Write};
use std::process::exit;

use chrono::Local;
use rand::prelude::*;

use crate::core::{Lesson, StudentRecord};

mod core;

fn main() {
	let student_record = StudentRecord::new(&LESSONS);
	let now = Local::now().timestamp();
	let mut rng = rand::thread_rng();
	let mut new_or_rested = student_record.new_or_rested_lessons(now);
	if new_or_rested.len() == 0 {
		println!("No active lessons, {} resting.", student_record.resting_lessons_count(now))
	} else {
		new_or_rested.shuffle(&mut rng);
		let lesson = new_or_rested[0];
		println!("RECALL: {}", match lesson { Lesson::Recall(_level, challenge, _solution) => challenge });
		print!("show|cancel: ");
		stdout().flush().unwrap();
		let input = {
			let mut input = String::new();
			std::io::stdin().read_line(&mut input).expect("Invalid command after challenge");
			input
		};
		if !input.starts_with("show") {
			exit(0)
		}
		println!("{}", match lesson { Lesson::Recall(_level, _challenge, solution) => solution });
		print!("repeat|pass|quit: ");
		stdout().flush().unwrap();
		let input = {
			let mut input = String::new();
			std::io::stdin().read_line(&mut input).expect("Invalid command after solution");
			input
		};
		println!("{}", input);
	}
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