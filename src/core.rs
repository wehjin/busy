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
		LessonRecord {
			lesson: lesson.to_owned(),
			pass_count: 0,
			pass_stamp: 0,
		}
	}
}
