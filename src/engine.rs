use chrono::{DateTime, Local};
use id::ID;
use logger::{Logger, StdoutLogger};
use object::Object;
use scene::Scene;
use std::collections::HashMap;

struct ObjectData {
	pub id: ID,
	pub tags: Vec<String>,
}

impl ObjectData {
	fn new(id: ID) -> Self {
		Self {
			id,
			tags: Vec::new(),
		}
	}

	fn get_ref(&self) -> Object {
		Object::new(self.id)
	}
}

pub struct Engine<L> {
	start_time: DateTime<Local>,
	last_obj_id: u32,
	objs: HashMap<ID, ObjectData>,
	scenes: HashMap<String, Scene>,
	pub logger: L,
}

impl Engine<StdoutLogger> {
	pub fn new() -> Self {
		Engine {
			start_time: Local::now(),
			last_obj_id: 0,
			objs: HashMap::new(),
			scenes: HashMap::new(),
			logger: StdoutLogger::default(),
		}
	}
}

impl<L> Engine<L>
where
	L: Logger, {
	pub fn new_object(&mut self) -> Object {
		let id = self.last_obj_id;
		let obj = ObjectData::new(id);
		self.objs.insert(id, obj);
		self.last_obj_id = self.last_obj_id + 1;

		self.objs.get(&id).unwrap().get_ref()
	}

	pub fn new_scene(&mut self, name: &str) -> &Scene {
		self.scenes.insert(name.to_owned(), Scene::new(name));

		self.scenes.get(name).unwrap()
	}
}
