use chrono::Duration;
use physics::PhysicsData;
use transform::Transformation;

#[derive(Copy, Clone, PartialEq)]
pub enum EntityType {
	Node,
	Transform(Transformation),
	Drawable,
	Physics(PhysicsData),
}

// DESIGN: Want game entities to be easily composable
// CONSIDERATION: Tree-like structure?
// PROS:
// CONS: Memory fragmentation
#[derive(Clone)]
pub struct Entity<'a> {
	name: Option<&'a str>,
	entity_type: EntityType,
	children: Vec<Entity<'a>>,
}

impl<'a> Entity<'a> {
	pub fn new(name: Option<&'a str>, entity_type: EntityType) -> Self {
		Self {
			entity_type,
			name,
			children: Vec::new(),
		}
	}

	pub fn physics(name: Option<&'a str>, pd: PhysicsData) -> Self {
		Self::new(name, EntityType::Physics(pd))
	}

	pub fn attach(&mut self, entity: Entity<'a>) {
		self.children.push(entity);
	}

	pub fn update(&mut self, timestep: Duration, mut transformation: Transformation) {
		println!("{}", self.name.unwrap_or(""));
		println!("{:?}", transformation.matrix());

		transformation = if let EntityType::Transform(t) = self.entity_type {
			transformation * t
		} else {
			transformation
		};

		self.children
			.iter_mut()
			.map(|e| e.update(timestep, transformation))
			.count();
	}
}
