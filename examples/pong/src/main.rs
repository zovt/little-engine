extern crate little_engine;

use little_engine::engine::Engine;
use little_engine::error::Error;
use little_engine::id::ID;
use little_engine::logger::Logger;
use little_engine::scene::Scene;

fn main() {
	let mut ngn = Engine::new();
	ngn.logger.info("Hello, World");
	ngn.logger.debug("Debug");
	ngn.logger.error(Error::new("Error"));

	let mut pong = ngn.new_scene("pong");
	let paddle1 = ngn.new_object();
	let paddle2 = ngn.new_object();
	let ball = ngn.new_object();

	pong.add_object(paddle1);
	pong.add_object(paddle2);
	pong.add_object(ball);

	/*
	pong::add_camera(camera);
	pong::set_active_camera(ID::String("view"));
	 */

	/* Potential API Design?
	let camera = Camera::new(ID::String("view"));

	ngn::add_scene(pong);
	match (ngn::load_scene(ID::String("pong"))) {
		Some(e) => ngn::error_log(e),
		None => (),
	};

	match (ngn::run()) {
		Some(e) => ngn::error_log(e),
		None => (),
	}
	 */
}
