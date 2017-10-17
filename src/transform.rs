use cgmath::Matrix4;
use cgmath::Zero;


pub struct Transform {
	pub matrix: Matrix4<f32>,
}

impl Default for Transform {
	fn default() -> Self {
		Transform {
			matrix: Matrix4::zero(),
		}
	}
}
