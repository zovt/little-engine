use chrono::Duration;

// CONSIDERATION: Event-based triggers vs. Tick-based updates?

pub trait Update {
	fn update(&mut self, timestep: Duration);
}
