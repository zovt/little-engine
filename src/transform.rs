use cgmath::{Matrix4, One, Vector3};


pub struct Transform {
	matrix: Matrix4<f32>,
	dirty: bool,

	position: Vector3<f32>,
}

impl Default for Transform {
	fn default() -> Self {
		Transform { matrix: Matrix4::one() }
	}
}
