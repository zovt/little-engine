use chrono::{DateTime, Local};
use glutin;

pub struct PhysicsEvent;
pub struct InputEvent;

pub enum EventType {
	Physics(PhysicsEvent),
	Input(InputEvent),
	Window(glutin::Event),
}

pub struct EventOptions;

pub struct Event {
	pub timestamp: DateTime<Local>,
	pub ev_type: EventType,
	pub ev_options: EventOptions,
}

impl Event {
	pub fn new(ev_type: EventType, ev_options: EventOptions) -> Self {
		Self {
			timestamp: Local::now(),
			ev_type,
			ev_options,
		}
	}
}

pub trait EventFilter {
	fn handles(&self, ev: Event) -> bool {
		false
	}
}

impl EventFilter for Fn(Event) -> bool {
	fn handles(&self, ev: Event) -> bool {
		self(ev)
	}
}

pub struct EventHandler {}
