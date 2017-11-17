use chrono::Duration;
use update::Update;

#[derive(Copy, Clone, Hash)]
pub enum EntityType {
	Entity,
	Drawable,
	Physics,
}

#[derive(Clone, Hash)]
pub struct Entity<'a> {
	name: Option<&'a str>,
	entity_type: EntityType,
	children: Vec<Entity<'a>>,
}

impl<'a> Update for Entity<'a> {
	fn update(&mut self, timestep: Duration) {
		println!("{}", self.name.unwrap_or(""));
		self.children.iter_mut().map(|e| e.update(timestep)).count();
	}
}

impl<'a> Entity<'a> {
	pub fn new(entity_type: EntityType, name: Option<&'a str>) -> Self {
		Self {
			entity_type,
			name,
			children: Vec::new(),
		}
	}
}
