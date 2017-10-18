use id::ID;

pub struct Scene {
	pub id: ID,
	pub objects: Vec<Object>,
}

impl Scene {
	pub fn new(id: ID) {
		Scene { id: ID,
		        objects: Vec::new(), }
	}
}
