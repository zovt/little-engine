#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}

extern crate cgmath;
extern crate glutin;
extern crate chrono;

pub mod object;
pub mod transform;
pub mod controller;
pub mod engine;
pub mod events;
pub mod scene;
pub mod error;
pub mod id;
pub mod logger;
