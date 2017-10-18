use chrono::Local;
use error::Error;

pub trait Logger {
	fn error(&self, e: Error);
	fn info(&self, msg: &str);
	fn debug(&self, msg: &str);
}

#[derive(Default)]
pub struct StdoutLogger {}

impl Logger for StdoutLogger {
	fn error(&self, e: Error) {
		println!("ERROR: {} [{}]", e.msg, e.time.time());
	}

	fn info(&self, msg: &str) {
		println!("INFO: {} [{}]", msg, Local::now().time());
	}

	fn debug(&self, msg: &str) {
		println!("DEBUG: {} [{}]", msg, Local::now().time());
	}
}
