use std::collections::HashMap;

use crate::core::{Difficulty, Lesson, StudentRecord};

#[derive(Debug, Clone)]
pub enum LaunchState {
	Empty { resting_count: usize },
	Ready { student_record: StudentRecord, now: i64 },
}

#[derive(Debug)]
pub enum LaunchAction {
	Close,
	Take,
	Record(HashMap<Lesson, Difficulty>),
}
