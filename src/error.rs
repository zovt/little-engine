use chrono::{DateTime, Local};
use glutin::CreationError;

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

impl From<CreationError> for Error {
	fn from(e: CreationError) -> Self {
		Error::new(&e.to_string())
	}
}
