use echo_lib::kv;

pub use lesson_record::*;
pub use student_record::*;

mod lesson_record;
mod student_record;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Lesson {
	Recall(usize, &'static str, &'static str)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Difficulty {
	Hard,
	Easy,
}

impl kv::Key for Lesson {}
