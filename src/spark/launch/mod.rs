use chrono::Local;
use echo_lib::kv;
use yui::prelude::*;

use crate::core::{Lesson, StudentRecord};
use crate::spark::QuizSpark;

pub use self::core::*;

mod core;

const TAKE_COUNT: usize = 3;

pub struct LaunchSpark {
	pub lessons: &'static [Lesson],
	pub kv_store: kv::Store,
}

impl Spark for LaunchSpark {
	type State = LaunchState;
	type Action = LaunchAction;
	type Report = ();

	fn create(&self, _ctx: &Create<Self::Action, Self::Report>) -> Self::State {
		self.fresh_state()
	}

	fn flow(&self, action: Self::Action, ctx: &impl Flow<Self::State, Self::Action, Self::Report>) -> AfterFlow<Self::State, Self::Report> {
		match action {
			LaunchAction::Close => AfterFlow::Close(None),
			LaunchAction::Take => {
				if let LaunchState::Ready { student_record, .. } = ctx.state() {
					let now = Local::now().timestamp();
					let lessons = student_record.next_lessons(TAKE_COUNT, now);
					let resting_count = student_record.resting_lessons_count(now);
					let quiz_spark = QuizSpark { lessons, resting_count };
					ctx.start_prequel(quiz_spark, ctx.link().callback(LaunchAction::Record));
				}
				AfterFlow::Ignore
			}
			LaunchAction::Record(results) => {
				if let LaunchState::Ready { student_record, .. } = ctx.state() {
					student_record.update(results).write(&self.kv_store);
					let state = self.fresh_state();
					AfterFlow::Revise(state)
				} else {
					AfterFlow::Ignore
				}
			}
		}
	}

	fn render(state: &Self::State, link: &Link<Self::Action>) -> Option<ArcYard> {
		let (status, button_data) = match state {
			LaunchState::Empty { resting_count } => (
				yard::label(format!("{} lessons resting, none active", resting_count), StrokeColor::BodyOnBackground, Cling::Center),
				vec![
					yard::button_enabled("Close", link.callback(move |_| LaunchAction::Close))
				]
			),
			LaunchState::Ready { student_record, now } => (
				{
					let ready_count = student_record.new_or_rested_lessons(*now).len();
					yard::label(format!("{} lessons ready", ready_count), StrokeColor::BodyOnBackground, Cling::Center)
				},
				vec![
					yard::button_enabled("Take 3", link.callback(move |_| LaunchAction::Take)),
					yard::button_enabled("Close", link.callback(move |_| LaunchAction::Close)),
				]
			),
		};
		let yard = status.pack_bottom(
			10,
			yard::trellis(1, 1, Cling::Top, button_data).confine_width(20, Cling::Center),
		);
		Some(yard)
	}
}

impl LaunchSpark {
	fn fresh_state(&self) -> LaunchState {
		let now = Local::now().timestamp();
		let kv_catalog = self.kv_store.catalog().unwrap();
		let student_record = StudentRecord::new(&self.lessons, &kv_catalog);
		if student_record.new_or_rested_lessons(now).is_empty() {
			LaunchState::Empty { resting_count: student_record.resting_lessons_count(now) }
		} else {
			LaunchState::Ready { student_record, now }
		}
	}
}
