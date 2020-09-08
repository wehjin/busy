use std::collections::HashMap;

use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use echo_lib::kv;
use rand::prelude::SliceRandom;

use crate::core::{Difficulty, Lesson, LessonRecord, PassInfo};

#[derive(Debug, Clone)]
pub struct StudentRecord {
	lesson_records: Vec<LessonRecord>
}

impl StudentRecord {
	pub fn new(lessons: &'static [Lesson], kv_catalog: &kv::Catalog) -> Self {
		let lesson_records = lessons.iter()
			.map(|it| {
				let lesson = it.clone();
				let pass_info = kv_catalog.read(it, || PassInfo::new()).expect("Catalog read failed");
				LessonRecord { lesson, pass_info }
			})
			.collect::<Vec<_>>();
		StudentRecord { lesson_records }
	}
	pub fn update(&self, results: HashMap<Lesson, Difficulty>) -> StudentRecord {
		let now = Local::now().timestamp();
		let lesson_records = self.lesson_records.iter()
			.map(|it| it.update_hashmap(&results, now))
			.collect::<Vec<_>>();
		StudentRecord { lesson_records }
	}
	pub fn write(&self, store: &kv::Store) {
		let catalog = store.catalog().expect("No catalog from store");
		self.lesson_records.iter().for_each(|it| {
			let lesson = &it.lesson;
			let stored_info = catalog.read(lesson, || PassInfo::new()).expect("Catalog read failed");
			if it.pass_info != stored_info {
				store.write(lesson, &it.pass_info).expect("Store write failed");
			}
		});
	}
	pub fn rest_end(&self) -> Option<DateTime<Local>> {
		let never = i64::max_value();
		let min_rest_end = self.lesson_records.iter().fold(never, |stamp, next_record| {
			stamp.min(next_record.rest_end())
		});
		if min_rest_end == never { None } else {
			let naive_end = NaiveDateTime::from_timestamp(min_rest_end, 0);
			let end = Local.from_utc_datetime(&naive_end);
			Some(end)
		}
	}
	pub fn resting_lessons_count(&self, now: i64) -> usize {
		self.lesson_records.iter().filter(|it| it.is_resting(now)).count()
	}
	pub fn new_or_rested_lessons(&self, now: i64) -> Vec<&Lesson> {
		self.lesson_records.iter()
			.filter(|it| it.is_new() || it.is_rested(now))
			.map(|it| &it.lesson).collect()
	}
	pub fn next_lessons(&self, count: usize, now: i64) -> Vec<Lesson> {
		let mut lessons = self.new_or_rested_lessons(now);
		lessons.shuffle(&mut rand::thread_rng());
		lessons.truncate(count);
		lessons.into_iter().cloned().collect()
	}
}
