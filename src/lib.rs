#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(plugin)]
#![cfg_attr(feature="clippy", feature(clippy))]
#![cfg_attr(feature="clippy", plugin(clippy))]

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

pub mod transform;
pub mod controller;
pub mod engine;
pub mod events;
pub mod error;
pub mod logger;
pub mod graphics;
pub mod entity;
pub mod scene;
pub mod util;
pub mod camera;
