use chrono::{DateTime, Duration, Local};
use scene::Scene;
use update::Update;

pub struct GameLoop {
	tickrate: u8, // ticks / second, in milliseconds
	framerate: Option<u8>, // frames / second. Unbounded if None
}

impl Default for GameLoop {
	fn default() -> Self {
		Self {
			tickrate: 60,
			framerate: None,
		}
	}
}

impl GameLoop {
	pub fn new(tickrate: u8, framerate: Option<u8>) -> Self {
		Self {
			tickrate,
			framerate,
		}
	}

	pub fn run(self, mut scene: Scene) {
		let mut ts = Local::now();
		let mut acc_t = Duration::zero();
		let dt_t = Duration::milliseconds(1000 / self.tickrate as i64);

		loop {
			let old_ts = ts;
			ts = Local::now();

			acc_t = acc_t + ts.signed_duration_since(old_ts);

			while acc_t >= dt_t {
				scene.update(dt_t);
				acc_t = acc_t - dt_t;
			}

			// TODO: LERP between states based on time left in acc
			// TODO: Render Scene
		}
	}
}
