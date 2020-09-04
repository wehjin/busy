use std::cmp::min;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Lesson {
	Recall(usize, &'static str, &'static str)
}

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
		let days = REST_DAYS[min(9, self.pass_count)];
		let hours = days * 24;
		self.pass_stamp + (hours - 1) * 3600
	}
}

#[derive(Debug)]
pub struct StudentRecord {
	lesson_records: Vec<LessonRecord>
}

impl StudentRecord {
	pub fn new(lessons: &[Lesson]) -> Self {
		StudentRecord {
			lesson_records: lessons.iter().map(LessonRecord::new).collect()
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
}
