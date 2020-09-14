use echo_lib::kv;

pub use lesson_record::*;
pub use student_record::*;

use crate::VocabRecord;

mod lesson_record;
mod student_record;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Lesson {
	StaticEnglishKana(usize, &'static str, &'static str),
	Dynamic(VocabRecord),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Difficulty {
	Hard,
	Easy,
}

impl kv::Key for Lesson {}
