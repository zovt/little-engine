extern crate little_engine;

use little_engine::entity::{Entity, EntityType};
use little_engine::scene::Scene;

fn main() {
	let left_paddle = Entity::new(EntityType::Live, Some("left paddle"));
	let right_paddle = Entity::new(EntityType::Live, Some("right paddle"));
	let ball = Entity::new(EntityType::Live, Some("ball"));
	// let camera = Camera::new();

	let mut pong_scene = Scene::default();
	pong_scene.entities.push(left_paddle);
	pong_scene.entities.push(right_paddle);
	pong_scene.entities.push(ball);

	/*
	let mut renderer = Renderer::new();
	let mut window_events = WindowEvents::new();
	let mut game_loop = GameLoop::new();

	game_loop.run(renderer, window_events, scene);
	*/
}
