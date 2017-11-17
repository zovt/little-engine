use scene::Scene;
use update::Update;

pub struct GameLoop {}

impl GameLoop {
	pub fn run(self, mut scene: Scene) {
		loop {
			scene.update();
		}
	}
}
