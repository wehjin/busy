use crate::core::Lesson;

#[derive(Debug, Clone)]
pub struct LessonRecord {
	pub lesson: Lesson,
	pub pass_count: usize,
	pub pass_stamp: i64,
}

impl LessonRecord {
	pub fn new(lesson: &Lesson) -> Self {
		LessonRecord { lesson: lesson.to_owned(), pass_count: 0, pass_stamp: 0 }
	}
	pub fn is_new(&self) -> bool { self.pass_count == 0 }
	pub fn is_rested(&self, now: i64) -> bool { self.pass_count > 0 && now > self.rest_end() }
	pub fn is_resting(&self, now: i64) -> bool { self.pass_count > 0 && now <= self.rest_end() }
	fn rest_end(&self) -> i64 {
		const REST_DAYS: [i64; 10] = [1, 2, 4, 8, 16, 32, 64, 128, 256, 512];
		let days = REST_DAYS[self.pass_count.min(REST_DAYS.len() - 1)];
		let hours = days * 24;
		self.pass_stamp + (hours - 1) * 3600
	}
}
