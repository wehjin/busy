use crate::core::{Lesson, LessonRecord};

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
