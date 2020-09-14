use std::error::Error;

use crate::core::Lesson;

#[cfg(test)]
mod tests;

#[derive(Debug, Deserialize, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "PascalCase")]
pub struct VocabRecord {
	pub ch: usize,
	pub pos: usize,
	pub en: String,
	pub jp: String,
}

pub fn fetch_lessons() -> Result<Vec<Lesson>, Box<dyn Error>> {
	let mut lessons = LESSONS.to_vec();
	let remotes = remote_lessons()?;
	lessons.extend(remotes);
	Ok(lessons)
}

fn remote_lessons() -> Result<Vec<Lesson>, Box<dyn Error>> {
	let vocab_records = vocab_records()?;
	let lessons = vocab_records.into_iter().map(Lesson::Dynamic).collect::<Vec<_>>();
	Ok(lessons)
}

fn vocab_records() -> Result<Vec<VocabRecord>, Box<dyn Error>> {
	let mut vocab_records: Vec<VocabRecord> = Vec::new();
	{
		let body = reqwest::blocking::get(URL)?.text()?;
		let mut rdr = csv::ReaderBuilder::new().from_reader(body.as_bytes());
		for result in rdr.deserialize() {
			let record: VocabRecord = result?;
			vocab_records.push(record);
		}
	}
	Ok(vocab_records)
}

pub const URL: &str = "https://docs.google.com/spreadsheets/d/e/2PACX-1vQC1PB5ovn2GXfTtzR7K0eCCnu819QxIFNqFD8RGM3XtMhrgApGOMVUs7JeBy_-318vgu1RrGHg8eMm/pub?gid=0&single=true&output=csv";
const LESSONS: [Lesson; 19] = [
	Lesson::StaticEnglishKana(2, "embassy", "たいしかん"),
	Lesson::StaticEnglishKana(12, "kind,generous", "しんせつ"),
	Lesson::StaticEnglishKana(13, "vase", "かびん"),
	Lesson::StaticEnglishKana(13, "rose", "ばら"),
	Lesson::StaticEnglishKana(13, "bouquet", "はなたば"),
	Lesson::StaticEnglishKana(13, "ring", "ゆびわ"),
	Lesson::StaticEnglishKana(13, "anniversary", "きねんび"),
	Lesson::StaticEnglishKana(13, "suit, look good on", "にあいます"),
	Lesson::StaticEnglishKana(14, "clearance sale", "バーゲンセール"),
	Lesson::StaticEnglishKana(15, "ticket gate", "かいさつぐち"),
	Lesson::StaticEnglishKana(15, "fireworks display", "はなびたいかい"),
	Lesson::StaticEnglishKana(15, "large gathering", "たいかい"),
	Lesson::StaticEnglishKana(15, "snow festival", "ゆきまつり"),
	Lesson::StaticEnglishKana(15, "game, match", "しあい"),
	Lesson::StaticEnglishKana(15, "platform(train)", "ホーム"),
	Lesson::StaticEnglishKana(15, "circumstances", "つごう"),
	Lesson::StaticEnglishKana(15, "Asakusa Station", "あさくさえき"),
	Lesson::StaticEnglishKana(15, "Shimbashi Station", "しんばしえき"),
	Lesson::StaticEnglishKana(15, "hall(concert)", "ホール"),
];
