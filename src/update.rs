use chrono::Duration;

pub trait Update {
	fn update(&mut self, timestep: Duration);
}
