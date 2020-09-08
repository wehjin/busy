use std::collections::HashMap;

use chrono::{DateTime, Local};

use crate::core::{Difficulty, Lesson, StudentRecord};

#[derive(Debug, Copy, Clone)]
pub enum RestStatus {
	Empty,
	Some { count: usize, end: DateTime<Local> },
}

#[derive(Debug, Clone)]
pub enum LaunchState {
	Empty { rest_status: RestStatus },
	Ready { student_record: StudentRecord, now: i64 },
}

#[derive(Debug)]
pub enum LaunchAction {
	Close,
	Take,
	Record(HashMap<Lesson, Difficulty>),
}
