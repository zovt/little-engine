use chrono::{DateTime, Local};
use error::Error;
use events::{Event, EventFilter, EventHandler, EventOptions, EventType, HandleEvent};
use glutin::{Event as GlutinEvent, WindowEvent};
use graphics::create_window;
use logger::{Logger, StdoutLogger};
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
		self.id.hash(state)
	}
}

impl<T> Clone for ID<T> {
	fn clone(&self) -> Self {
		Self::new(self.id)
	}
}

impl<T> PartialEq for ID<T> {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
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
	pub event_handlers: Vec<EventHandler<Box<EventFilter>, Box<HandleEvent>>>,
}

impl Scene {
	fn new(id: ID<Scene>, name: &str) -> Self {
		Self {
			id,
			name: name.to_owned(),
			objects: HashSet::new(),
			event_handlers: Vec::new(),
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

	pub fn acquire<T, F: FnOnce(&mut D) -> T>(&mut self, item: ID<D>, f: F) -> Option<T> {
		self.items.get_mut(&item).and_then(|item| Some(f(item)))
	}
}

// TODO: Managers should reuse object IDs
impl Manager<Object> {
	pub fn create(&mut self) -> ID<Object> {
		let obj = Object::new(self.fresh_id);
		let id = obj.id;
		self.items.insert(obj.id, obj);

		self.fresh_id.id += 1;
		self.items[&id].id
	}
}

impl Manager<Scene> {
	pub fn create(&mut self, name: &str) -> ID<Scene> {
		let s = Scene::new(self.fresh_id, name);
		let id = s.id;
		self.items.insert(s.id, s);

		self.fresh_id.id += 1;
		self.items[&id].id
	}
}

pub struct Engine<L> {
	start_time: DateTime<Local>,
	running: bool,
	pub objects: Manager<Object>,
	pub scenes: Manager<Scene>,
	pub logger: L,
	active_scene: Option<ID<Scene>>,
	events: Vec<Event>,
}

impl Default for Engine<StdoutLogger> {
	fn default() -> Self {
		Engine {
			start_time: Local::now(),
			running: false,
			objects: Manager::new(),
			scenes: Manager::new(),
			logger: StdoutLogger::default(),
			active_scene: None,
			events: Vec::new(),
		}
	}
}

impl<L> Engine<L>
where
	L: Logger, {
	pub fn set_active_scene(&mut self, scene: ID<Scene>) {
		self.active_scene = Some(scene);
	}

	fn process_glutin_events(&mut self, ev: GlutinEvent) {
		if let GlutinEvent::WindowEvent { event: WindowEvent::Closed, .. } = ev {
			self.running = false;
			return;
		}

		self.events
			.push(Event::new(EventType::Window(ev), EventOptions { single_use: false }));
	}

	fn update(&mut self) -> Result<(), Error> {
		if let Some(scene_h) = self.active_scene {
			let events: Vec<_> = self.events.drain(..).collect();

			self.scenes
				.acquire(scene_h, |scene| {
					events
						.iter()
						.flat_map(|ev| {
							scene
								.event_handlers
								.iter()
								.filter(|eh| eh.filter.handles(ev))
								.flat_map(|eh| eh.handler.handle(ev))
								.collect::<Vec<Event>>()
						})
						.collect::<Vec<Event>>()
				})
				.map(|mut evs| self.events.append(&mut evs))
				.map_or(Err(Error::new("Missing Scene")), Ok)
		} else {
			Ok(())
		}
	}

	fn draw(&mut self) {}

	pub fn run(&mut self, name: &str) -> Result<(), Error> {
		let (_, mut ev_loop) = create_window(name)?;

		self.running = true;
		while self.running {
			ev_loop.poll_events(|ev| self.process_glutin_events(ev));
			self.update().and_then(|_| Ok(self.draw()))?
		}

		Ok(())
	}
}
