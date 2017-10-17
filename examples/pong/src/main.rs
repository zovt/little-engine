extern crate little_engine;

use little_engine::objects::WorldObject;

fn main() {
	let engine = Engine::new();
	let paddle1 = Engine::build_object().with_drawable().build();
	let paddle2 = Engine::clone(paddle1);

	let paddle1 = WorldObject::default();
	let paddle1 = WorldObject::default();
}
