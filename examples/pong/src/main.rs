extern crate little_engine;

use little_engine::engine::Engine;
use little_engine::error::Error;
use little_engine::logger::Logger;

fn main() {
	let mut ngn = Engine::new();
	ngn.logger.info("Hello, World");
	ngn.logger.debug("Debug");
	ngn.logger.error(Error::new("Error"));

	let pong = ngn.scenes.create("pong");
	let paddle1 = ngn.objects.create();
	let paddle2 = ngn.objects.create();
	let ball = ngn.objects.create();

	ngn.scenes.acquire(pong, |p| {
		p.add_object(paddle1);
		p.add_object(paddle1);
		p.add_object(paddle2);
		p.add_object(ball);
	});

	ngn.load_scene(pong).map(|e| ngn.logger.error(e));

	/*
	pong::add_camera(camera);
	pong::set_active_camera(ID::String("view"));
	 */

	/* Potential API Design?
	let camera = Camera::new(ID::String("view"));


	match (ngn::run()) {
		Some(e) => ngn::error_log(e),
		None => (),
	}
	 */
}
