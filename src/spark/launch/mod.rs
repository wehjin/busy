use chrono::Local;
use yui::prelude::*;

use crate::core::StudentRecord;
use crate::spark::QuizSpark;

pub use self::core::*;

mod core;

const TAKE_COUNT: usize = 3;

pub struct LaunchSpark {
	pub student_record: StudentRecord
}

impl Spark for LaunchSpark {
	type State = LaunchState;
	type Action = LaunchAction;
	type Report = ();

	fn create(&self, _ctx: &Create<Self::Action, Self::Report>) -> Self::State {
		let now = Local::now().timestamp();
		let new_or_rested_lessons = self.student_record.new_or_rested_lessons(now);
		if new_or_rested_lessons.is_empty() {
			let resting_count = self.student_record.resting_lessons_count(now);
			LaunchState::Empty { resting_count }
		} else {
			let new_or_rested_count = new_or_rested_lessons.len();
			LaunchState::Ready { new_or_rested_count }
		}
	}

	fn flow(&self, action: Self::Action, ctx: &impl Flow<Self::State, Self::Action, Self::Report>) -> AfterFlow<Self::State, Self::Report> {
		match action {
			LaunchAction::Close => AfterFlow::Close(None),
			LaunchAction::Take => {
				let now = Local::now().timestamp();
				let lessons = self.student_record.next_lessons(TAKE_COUNT, now);
				let resting_count = self.student_record.resting_lessons_count(now);
				let quiz_spark = QuizSpark { lessons, resting_count };
				ctx.start_prequel(quiz_spark, |_| {});
				AfterFlow::Ignore
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
			LaunchState::Ready { new_or_rested_count } => (
				yard::label(format!("{} lessons ready", new_or_rested_count), StrokeColor::BodyOnBackground, Cling::Center),
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

