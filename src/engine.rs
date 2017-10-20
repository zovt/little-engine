use chrono::{DateTime, Local};
use error::Error;
use logger::{Logger, StdoutLogger};
use std::clone::Clone;
use std::cmp::{Eq, PartialEq};
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::marker::{Copy, PhantomData};

pub struct ID<T> {
	id: u32,
	_t: PhantomData<T>,
}

impl<T> ID<T> {
	pub fn new(id: u32) -> Self {
		Self {
			id,
			_t: PhantomData::default(),
		}
	}
}

impl<T> Hash for ID<T> {
	fn hash<H>(&self, state: &mut H)
	where
		H: Hasher, {
		self.id.hash(state);
	}
}

impl<T> PartialEq for ID<T> {
	fn eq(&self, other: &Self) -> bool {
		self.id.eq(&other.id)
	}
}

impl<T> Clone for ID<T> {
	fn clone(&self) -> Self {
		Self {
			id: self.id,
			_t: PhantomData::default(),
		}
	}
}

impl<T> Eq for ID<T> {}
impl<T> Copy for ID<T> {}

pub struct Object {
	pub id: ID<Object>,
	pub tags: Vec<String>,
}

impl Object {
	fn new(id: ID<Object>) -> Self {
		Self {
			id,
			tags: Vec::new(),
		}
	}
}

pub struct Scene {
	pub id: ID<Scene>,
	pub name: String,
	pub objects: HashSet<ID<Object>>,
}

impl Scene {
	fn new(id: ID<Scene>, name: &str) -> Self {
		Self {
			id,
			name: name.to_owned(),
			objects: HashSet::new(),
		}
	}

	pub fn add_object(&mut self, object: ID<Object>) -> Option<ID<Object>> {
		self.objects.replace(object)
	}
}

pub struct Manager<D> {
	fresh_id: ID<D>,
	items: HashMap<ID<D>, D>,
}

impl<D> Manager<D> {
	fn new() -> Self {
		Self {
			fresh_id: ID::new(0),
			items: HashMap::new(),
		}
	}

	pub fn delete(&mut self, item: ID<D>) {
		self.items.remove(&item);
	}

	pub fn acquire<F: FnOnce(&mut D)>(&mut self, item: ID<D>, f: F) -> bool {
		self.items
			.get_mut(&item)
			.and_then(|item| {
				f(item);
				Some(())
			})
			.is_some()
	}
}

impl Manager<Object> {
	pub fn create(&mut self) -> ID<Object> {
		let obj = Object::new(self.fresh_id);
		let id = obj.id;
		self.items.insert(obj.id, obj);

		self.fresh_id.id = self.fresh_id.id + 1;
		self.items.get(&id).unwrap().id
	}
}

impl Manager<Scene> {
	pub fn create(&mut self, name: &str) -> ID<Scene> {
		let s = Scene::new(self.fresh_id, name);
		let id = s.id;
		self.items.insert(s.id, s);

		self.fresh_id.id = self.fresh_id.id + 1;
		self.items.get(&id).unwrap().id
	}
}

pub struct Engine<L> {
	start_time: DateTime<Local>,
	pub objects: Manager<Object>,
	pub scenes: Manager<Scene>,
	pub logger: L,
}

impl Engine<StdoutLogger> {
	pub fn new() -> Self {
		Engine {
			start_time: Local::now(),
			objects: Manager::new(),
			scenes: Manager::new(),
			logger: StdoutLogger::default(),
		}
	}
}

impl<L> Engine<L>
where
	L: Logger, {
	pub fn load_scene(&mut self, scene: ID<Scene>) -> Option<Error> {
		match self.scenes.items.get(&scene) {
			None => Some(Error::new("Missing scene")),
			_ => None,
		}
	}

	pub fn run() -> Option<Error> {
		None
	}
}
