extern crate little_engine;

use little_engine::engine::Engine;
use little_engine::id::ID;

fn main() {
	let ngn = Engine::new();
	ngn.log_info("Hello, World");

	/*
	let pong = Scene::new(ID::String("pong"));

	let paddle1 = Object::new();
	let paddle2 = Object::new();
	let ball = Object::new();

	let camera = Camera::new(ID::String("view"));

	pong::add_object(paddle1);
	pong::add_object(paddle2);
	pong::add_object(ball);
	pong::add_camera(camera);
	pong::set_active_camera(ID::String("view"));

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
