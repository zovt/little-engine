use cgmath::{Vector2, Vector3};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Square {
	pub width: f32,
	pub height: f32,
}

impl Into<Vector2<f32>> for Square {
	fn into(self) -> Vector2<f32> {
		Vector2 {
			x: self.width,
			y: self.height,
		}
	}
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Box {
	pub width: f32,
	pub height: f32,
	pub depth: f32,
}

impl Into<Vector3<f32>> for Box {
	fn into(self) -> Vector3<f32> {
		Vector3 {
			x: self.width,
			y: self.height,
			z: self.depth,
		}
	}
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct PhysicsData {}
