use camera::Camera;
use entity::Entity;

#[derive(Default)]
pub struct Scene<'a> {
	pub entities: Vec<Entity<'a>>,
	pub cameras: Vec<Camera>,
}
