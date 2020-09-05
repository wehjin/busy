use std::collections::HashMap;

use chrono::Local;
use rand::prelude::*;
use yui::Before;
use yui::prelude::*;

use crate::core::{Difficulty, Lesson, StudentRecord};

const SIDE_WIDTH: i32 = 25;

#[derive(Debug)]
pub struct MainSpark {
	pub student_record: StudentRecord
}

#[derive(Debug, Clone)]
pub struct MainState {
	resting_count: usize,
	lessons: Vec<Lesson>,
	lesson_index: usize,
	check_answer: bool,
	results: HashMap<Lesson, Difficulty>,
}

#[derive(Debug)]
pub enum MainAction {
	Quit,
	CheckAnswer,
	Back,
	Space,
}

impl Spark for MainSpark {
	type State = MainState;
	type Action = MainAction;
	type Report = HashMap<Lesson, Difficulty>;

	fn create(&self, _ctx: &Create<Self::Action, Self::Report>) -> Self::State {
		let now = Local::now().timestamp();
		let lessons = next_lessons(3, now, &self.student_record);
		let resting_count = self.student_record.resting_lessons_count(now);
		MainState { resting_count, lessons, lesson_index: 0, check_answer: false, results: HashMap::new() }
	}

	fn flow(&self, action: Self::Action, ctx: &impl Flow<Self::State, Self::Action, Self::Report>) -> AfterFlow<Self::State, Self::Report> {
		match action {
			MainAction::Quit => AfterFlow::Close(None),
			MainAction::CheckAnswer => {
				let mut state = ctx.state().clone();
				state.check_answer = true;
				AfterFlow::Revise(state)
			}
			MainAction::Back => {
				let mut state = ctx.state().clone();
				state.check_answer = false;
				AfterFlow::Revise(state)
			}
			MainAction::Space => {
				let mut state = ctx.state().clone();
				let spaced_lesson = state.lessons.remove(state.lesson_index);
				state.lesson_index = if state.lessons.is_empty() { 0 } else { state.lesson_index % state.lessons.len() };
				state.check_answer = false;
				if !state.results.contains_key(&spaced_lesson) {
					state.results.insert(spaced_lesson, Difficulty::Easy);
				}
				if state.lessons.is_empty() {
					AfterFlow::Close(Some(state.results))
				} else {
					AfterFlow::Revise(state)
				}
			}
		}
	}

	fn render(state: &Self::State, link: &Link<Self::Action>) -> Option<ArcYard> {
		if state.lessons.len() == 0 {
			let text = format!("No active lessons. {} resting.", state.resting_count);
			Some(yard::label(text, StrokeColor::CommentOnBackground, Cling::Center))
		} else {
			let yard = if state.check_answer {
				let content_yard = solution_body_yard(state);
				let side_yard = side_yard(state, vec![
					yard::button_enabled("Back", link.callback(move |_| MainAction::Back)),
					yard::button_enabled("Repeat", link.callback(move |_| MainAction::Back)),
					yard::button_enabled("Space", link.callback(move |_| MainAction::Space)),
				]);
				content_yard.pack_right(SIDE_WIDTH, side_yard)
			} else {
				let content_yard = challenge_body_yard(state);
				let side_yard = side_yard(state, vec![
					yard::button_enabled("Check Answer", link.callback(move |_| MainAction::CheckAnswer)),
					yard::button_enabled("Quit", link.callback(move |_| MainAction::Quit)),
				]);
				content_yard.pack_right(SIDE_WIDTH, side_yard)
			};
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

fn solution_body_yard(state: &MainState) -> ArcYard {
	let lesson = &state.lessons[state.lesson_index];
	match lesson {
		&Lesson::Recall(_, _, solution) => {
			let text = format!("{}", solution);
			yard::label(text, StrokeColor::BodyOnBackground, Cling::Center)
		}
	}
}

fn challenge_body_yard(state: &MainState) -> ArcYard {
	let lesson = &state.lessons[state.lesson_index];
	match lesson {
		&Lesson::Recall(_, challenge, _) => {
			let text = format!("{}", challenge);
			yard::label(text, StrokeColor::BodyOnBackground, Cling::Center)
		}
	}
}

fn side_yard(state: &MainState, button_yards: Vec<ArcYard>) -> ArcYard {
	let position_label = {
		let text = format!("Lesson {} of {}", state.lesson_index + 1, state.lessons.len());
		yard::label(text, StrokeColor::CommentOnBackground, Cling::RightTop)
	};
	let button_section = yard::trellis(1, 1, Cling::Right, button_yards)
		.confine_width(SIDE_WIDTH - 6, Cling::Center);
	let front = button_section.pack_top(2, position_label);
	front.before(yard::fill(FillColor::Primary)).pad(1)
}
