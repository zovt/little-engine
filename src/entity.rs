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
	fn update(&mut self) {
		println!("{}", self.name.unwrap_or(""));
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
