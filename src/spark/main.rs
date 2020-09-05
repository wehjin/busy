use chrono::Local;
use rand::prelude::*;
use yui::Before;
use yui::prelude::*;

use crate::core::{Lesson, StudentRecord};

#[derive(Debug)]
pub struct MainSpark {
	pub student_record: StudentRecord
}

#[derive(Debug, Clone)]
pub struct MainState {
	resting_count: usize,
	lessons: Vec<Lesson>,
	lessons_index: usize,
}

const SIDE_WIDTH: i32 = 20;

impl Spark for MainSpark {
	type State = MainState;
	type Action = ();
	type Report = ();

	fn create(&self, _ctx: &Create<Self::Action, Self::Report>) -> Self::State {
		let now = Local::now().timestamp();
		let lessons = next_lessons(3, now, &self.student_record);
		let resting_count = self.student_record.resting_lessons_count(now);
		MainState { resting_count, lessons, lessons_index: 0 }
	}

	fn flow(&self, _action: Self::Action, _ctx: &impl Flow<Self::State, Self::Action, Self::Report>) -> AfterFlow<Self::State, Self::Report> {
		AfterFlow::Ignore
	}

	fn render(state: &Self::State, _link: &Link<Self::Action>) -> Option<ArcYard> {
		if state.lessons.len() == 0 {
			let text = format!("No active lessons. {} resting.", state.resting_count);
			Some(yard::label(text, StrokeColor::CommentOnBackground, Cling::Center))
		} else {
			let content_yard = challenge_body_yard(state);
			let yard = content_yard.pack_right(SIDE_WIDTH, challenge_side_yard(state));
			Some(yard)
		}
	}
}

fn next_lessons(count: usize, now: i64, student_record: &StudentRecord) -> Vec<Lesson> {
	let mut rng = rand::thread_rng();
	let mut new_or_rested = student_record.new_or_rested_lessons(now);
	if new_or_rested.len() > count {
		new_or_rested.shuffle(&mut rng);
		new_or_rested.truncate(count);
	}
	new_or_rested.into_iter().cloned().collect()
}

fn challenge_body_yard(state: &MainState) -> ArcYard {
	let lesson = &state.lessons[state.lessons_index];
	match lesson {
		&Lesson::Recall(_, challenge, _) => {
			let text = format!("{}", challenge);
			yard::label(text, StrokeColor::BodyOnBackground, Cling::Center)
		}
	}
}

fn challenge_side_yard(state: &MainState) -> ArcYard {
	let position_label = {
		let text = format!("Lesson {} of {}", state.lessons_index + 1, state.lessons.len());
		yard::label(text, StrokeColor::CommentOnBackground, Cling::RightTop)
	};
	let button_cluster = {
		yard::trellis(1, 1, vec![
			yard::button_enabled("Check Answer", |_| {}),
			yard::button_enabled("Quit", |_| {}),
		]).confine_height(3, Cling::Center)
	}.confine_width(SIDE_WIDTH - 2, Cling::Right);
	let front = button_cluster.pack_top(2, position_label);
	front.before(yard::fill(FillColor::Primary))
}


