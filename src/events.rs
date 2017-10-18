use std::time::Instant;

pub enum EventType {
	Collision(/* Obj1, Obj2 */),
	Custom(String),
}

pub struct Event {
	pub timestamp: Instant,
	pub e_type: EventType,
}
