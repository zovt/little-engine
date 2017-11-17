use camera::Camera;
use entity::Entity;
use update::Update;

#[derive(Default)]
pub struct Scene<'a> {
	pub entities: Vec<Entity<'a>>,
	pub cameras: Vec<Camera>,
}

impl<'a> Update for Scene<'a> {
	fn update(&mut self) {
		self.entities.iter_mut().map(|e| e.update()).count();
	}
}
