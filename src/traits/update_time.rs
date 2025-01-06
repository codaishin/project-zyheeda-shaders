use std::time::Duration;

pub trait UpdateTime {
	fn update_time(&mut self, time: Duration);
}
