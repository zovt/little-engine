use transform::Transform;

#[derive(Default)]
pub struct WorldObject {
	pub transform: Transform,
}

impl Into<Transform> for WorldObject {
	fn into(self) -> Transform {
		return self.transform;
	}
}
