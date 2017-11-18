use cgmath::{Matrix4, Rad, Transform, Vector3, Vector4};
use cgmath::prelude::*;

#[derive(Copy, Clone, PartialEq)]
pub struct Transformation {
	matrix: Matrix4<f32>,
	dirty: bool,

	position: Vector3<f32>,
	scale: (f32, f32, f32),
	rotation: (Rad<f32>, Rad<f32>, Rad<f32>),
}

impl Default for Transformation {
	fn default() -> Self {
		Self {
			matrix: <Matrix4<f32> as One>::one(),
			dirty: true,
			position: Vector3::zero(),
			scale: (1f32, 1f32, 1f32),
			rotation: (Rad(0f32), Rad(0f32), Rad(0f32)),
		}
	}
}

impl Transformation {
	pub fn transform(&mut self) -> Matrix4<f32> {
		if self.dirty {
			self.update_matrix();
		}

		self.matrix
	}

	fn update_matrix(&mut self) {
		self.matrix = <Matrix4<f32> as One>::one();

		// Translate
		self.matrix = self.matrix * Matrix4::from_translation(self.position);

		// Rotate
		let r_y = Matrix4::from_angle_y(self.rotation.1);
		let axis_xp = (Matrix4::from_angle_y(self.rotation.1) * Vector4::unit_x())
			.truncate();
		let r_xp = Matrix4::from_axis_angle(axis_xp, self.rotation.0);
		let axis_zp = (Matrix4::from_angle_y(self.rotation.1) *
			               Matrix4::from_axis_angle(axis_xp, self.rotation.0) *
			               Vector4::unit_z())
			.truncate();
		let r_zp = Matrix4::from_axis_angle(axis_zp, self.rotation.2);

		self.matrix = self.matrix * r_y * r_xp * r_zp;

		// Scale
		let scale = Matrix4::from_nonuniform_scale(self.scale.0, self.scale.1, self.scale.2);
		self.matrix = self.matrix * scale;

		self.dirty = false;
	}

	pub fn move_to(self, x: f32, y: f32, z: f32) -> Self {
		self.dirty = true;
		self.position.x = x;
		self.position.y = y;
		self.position.z = z;
		self
	}

	pub fn move_to_x(&mut self, x: f32) -> Self {
		self.move_to(x, self.position.y, self.position.z)
	}

	pub fn move_to_y(&mut self, y: f32) -> Self {
		self.move_to(self.position.x, y, self.position.z)
	}

	pub fn move_to_z(&mut self, z: f32) -> Self {
		self.move_to(self.position.x, self.position.y, z)
	}

	pub fn move_by(self, x: f32, y: f32, z: f32) -> Self {
		self.move_to(
			self.position.x + x,
			self.position.y + y,
			self.position.z + z,
		)
	}

	pub fn move_by_x(&mut self, x: f32) -> Self {
		self.move_by(x, self.position.y, self.position.z)
	}

	pub fn move_by_y(&mut self, y: f32) -> Self {
		self.move_by(self.position.x, y, self.position.z)
	}

	pub fn move_by_z(&mut self, z: f32) -> Self {
		self.move_by(self.position.x, self.position.y, z)
	}

	pub fn rotate_to<X, Y, Z>(self, x: X, y: Y, z: Z) -> Self
	where
		X: Into<Rad<f32>>,
		Y: Into<Rad<f32>>,
		Z: Into<Rad<f32>>, {
		self.dirty = true;
		self.rotation.0 = x.into();
		self.rotation.1 = y.into();
		self.rotation.2 = z.into();

		self
	}

	pub fn rotate_to_x<T>(self, x: T) -> Self
	where
		T: Into<Rad<f32>>, {
		self.rotate_to(x, self.rotation.1, self.rotation.2)
	}

	pub fn rotate_to_y<T>(self, y: T) -> Self
	where
		T: Into<Rad<f32>>, {
		self.rotate_to(self.rotation.0, y, self.rotation.2)
	}

	pub fn rotate_to_z<T>(self, z: T) -> Self
	where
		T: Into<Rad<f32>>, {
		self.rotate_to(self.rotation.0, self.rotation.1, z)
	}

	pub fn rotate_by<X, Y, Z>(self, x: X, y: Y, z: Z) -> Self
	where
		X: Into<Rad<f32>>,
		Y: Into<Rad<f32>>,
		Z: Into<Rad<f32>>, {
		self.rotate_to(
			self.rotation.0 + x.into(),
			self.rotation.1 + y.into(),
			self.rotation.2 + z.into(),
		)
	}

	pub fn rotate_by_x<T>(self, x: T) -> Self
	where
		T: Into<Rad<f32>>, {
		self.rotate_by(x, self.rotation.1, self.rotation.2)
	}

	pub fn rotate_by_y<T>(self, y: T) -> Self
	where
		T: Into<Rad<f32>>, {
		self.rotate_by(self.rotation.0, y, self.rotation.2)
	}

	pub fn rotate_by_z<T>(self, z: T) -> Self
	where
		T: Into<Rad<f32>>, {
		self.rotate_by(self.rotation.0, self.rotation.1, z)
	}

	pub fn scale_to(self, x: f32, y: f32, z: f32) -> Self {
		self.dirty = true;
		self.scale.0 = x;
		self.scale.1 = y;
		self.scale.2 = z;
		self
	}

	pub fn scale_to_x(self, x: f32) -> Self {
		self.scale_to(x, self.scale.1, self.scale.2)
	}

	pub fn scale_to_y(self, y: f32) -> Self {
		self.scale_to(self.scale.0, y, self.scale.2)
	}

	pub fn scale_to_z(self, z: f32) -> Self {
		self.scale_to(self.scale.0, self.scale.1, z)
	}

	pub fn scale_by(self, x: f32, y: f32, z: f32) -> Self {
		self.scale_to(self.scale.0 + x, self.scale.1 + y, self.scale.2 + z)
	}

	pub fn scale_by_x(self, x: f32) -> Self {
		self.scale_by(x, self.scale.1, self.scale.2)
	}

	pub fn scale_by_y(self, y: f32) -> Self {
		self.scale_by(self.scale.0, y, self.scale.2)
	}

	pub fn scale_by_z(self, z: f32) -> Self {
		self.scale_by(self.scale.0, self.scale.1, z)
	}
}
