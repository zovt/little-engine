use error::Error;
use std::io::{BufWriter, stdout};
use std::time::Instant;

pub struct Engine<'a> {
	start_time: Instant,
	error_log: &'a mut BufWriter,
	info_log: &'a mut BufWriter,
	debug_log: &'a mut BufWriter,
}

impl<'a> Engine<'a> {
	pub fn new() -> Self {
		Engine {
			start_time: Instant::now(),
			error_log: stdout(),
			info_log: stdout(),
			debug_log: stdout(),
		}
	}

	pub fn log_error(&self, e: &Error) {
		write!(
			&mut self.error_log,
			"ERROR: {} {:?}\n",
			e.msg,
			e.time.duration_since(self.start_time)
		);
	}

	pub fn log_info(&self, msg: &str) {
		writeln!(
			&mut self.info_log,
			"INFO: {} {:?}",
			msg,
			Instant::now().duration_since(self.start_time)
		);
	}

	pub fn log_debug(&self, msg: &str) {
		writeln!(
			&mut self.debug_log,
			"DEBUG: {} {:?}",
			msg,
			Instant::now().duration_since(self.start_time)
		);
	}
}
