#[derive(Copy, Clone, Hash)]
pub enum EntityType {
	Live,
}

#[derive(Copy, Clone, Hash)]
pub struct Entity<'a> {
	name: Option<&'a str>,
	entity_type: EntityType,
}

impl<'a> Entity<'a> {
	pub fn new(entity_type: EntityType, name: Option<&'a str>) -> Self {
		Self { entity_type, name }
	}
}
