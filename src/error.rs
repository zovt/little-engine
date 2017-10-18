use std::time::Instant;

pub struct Error {
	pub time: Instant,
	pub msg: String,
}
