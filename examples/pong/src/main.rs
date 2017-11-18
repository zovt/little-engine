extern crate little_engine;

use little_engine::entity::{Entity, EntityType};
use little_engine::gameloop::GameLoop;
use little_engine::physics::PhysicsData;
use little_engine::scene::Scene;

fn main() {
	let paddle_physics = Entity::physics(Some("paddle physics"), PhysicsData {});

	let mut left_paddle = Entity::new(Some("left paddle"), EntityType::Node);
	left_paddle.attach(paddle_physics.clone());

	let mut right_paddle = Entity::new(Some("right paddle"), EntityType::Node);
	right_paddle.attach(paddle_physics.clone());

	let ball_physics = Entity::physics(Some("ball physics"), PhysicsData {});
	let mut ball = Entity::new(Some("ball"), EntityType::Node);
	ball.attach(ball_physics);

	// TODO: Top and Bottom walls
	// TODO: "Field"
	// TODO: Score zones
	// TODO: Score counters
	// let camera = Camera::new();

	let mut pong_scene = Scene::default();
	pong_scene.entities.push(left_paddle);
	pong_scene.entities.push(right_paddle);
	pong_scene.entities.push(ball);

	/*
	let mut renderer = Renderer::new();
	let mut window_events = WindowEvents::new();
	 */

	let mut game_loop = GameLoop::default();
	game_loop.run(pong_scene);
}
