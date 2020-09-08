use std::collections::HashMap;
use std::error::Error;

use echo_lib::kv;
use serde::{Deserialize, Serialize};

use crate::core::{Difficulty, Lesson};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct PassInfo {
	pub count: usize,
	pub stamp: i64,
}

impl kv::Value for PassInfo {
	fn to_value_string(&self) -> String {
		serde_json::to_string(self).unwrap()
	}

	fn from_value_string(s: &String) -> Result<Self, Box<dyn Error>> {
		Ok(serde_json::from_str::<PassInfo>(s)?)
	}
}

impl PassInfo {
	pub fn new() -> Self { PassInfo { count: 0, stamp: 0 } }
}

#[derive(Debug, Clone)]
pub struct LessonRecord {
	pub lesson: Lesson,
	pub pass_info: PassInfo,
}

impl LessonRecord {
	pub fn update_hashmap(&self, difficulty_map: &HashMap<Lesson, Difficulty>, pass_stamp: i64) -> LessonRecord {
		match difficulty_map.get(&self.lesson) {
			None => self.clone(),
			Some(difficulty) => self.update(difficulty, pass_stamp),
		}
	}
	fn update(&self, difficulty: &Difficulty, pass_stamp: i64) -> LessonRecord {
		let lesson = self.lesson.clone();
		let pass_count = match difficulty {
			Difficulty::Hard => 1,
			Difficulty::Easy => self.pass_info.count + 1,
		};
		LessonRecord { lesson, pass_info: PassInfo { count: pass_count, stamp: pass_stamp } }
	}
	pub fn is_new(&self) -> bool { self.pass_info.count == 0 }
	pub fn is_rested(&self, now: i64) -> bool { self.pass_info.count > 0 && now > self.rest_end() }
	pub fn is_resting(&self, now: i64) -> bool { self.pass_info.count > 0 && now <= self.rest_end() }
	pub fn rest_end(&self) -> i64 {
		const REST_DAYS: [i64; 10] = [1, 2, 4, 8, 16, 32, 64, 128, 256, 512];
		let pass_count = self.pass_info.count;
		if pass_count == 0 {
			0
		} else {
			let rest_days_index = (pass_count - 1).min(REST_DAYS.len() - 1);
			let hours = REST_DAYS[rest_days_index] * 24;
			self.pass_info.stamp + (hours - 1) * 3600
		}
	}
}
