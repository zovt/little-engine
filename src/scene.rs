use object::Object;

pub struct Scene {
	pub name: String,
	pub objects: Vec<Object>,
}

impl Scene {
	pub fn new(name: &str) -> Self {
		Self {
			name: name.to_owned(),
			objects: Vec::new(),
		}
	}

	pub fn add_object(&mut self, obj: Object) {
		self.objects.push(obj);
	}
}
