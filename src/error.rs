use chrono::{DateTime, Local};

#[derive(Clone)]
pub struct Error {
	pub time: DateTime<Local>,
	pub msg: String,
}

impl Error {
	pub fn new(msg: &str) -> Self {
		Self {
			time: Local::now(),
			msg: msg.to_owned(),
		}
	}
}
