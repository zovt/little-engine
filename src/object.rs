use id::ID;

#[derive(Copy, Clone)]
pub struct Object {
	id: ID,
}

impl Object {
	pub fn new(id: ID) -> Self {
		Self { id }
	}
}
