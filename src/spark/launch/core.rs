#[derive(Debug, Clone)]
pub enum LaunchState {
	Empty { resting_count: usize },
	Ready { new_or_rested_count: usize },
}

#[derive(Debug)]
pub enum LaunchAction {
	Close,
	Take,
}
