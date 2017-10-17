use std::collections::HashMap;

use glutin::WindowEvent;

use transform::Transform;

pub enum MouseInput {
	Moved(WindowEvent::MouseMoved),
	Wheel(WindowEvent::MouseWheel),
	Left(WindowEvent::MouseLeft),
	Right(WindowEvent::MouseRight),
	Input(WindowEvent::MouseInput),
}

pub enum UserInput {
	Keyboard(WindowEvent::KeyboardInput),
	Mouse(),
}

pub struct InputMap<T, R, F>
	where F: Fn(T) -> R {
	pub map: HashMap<VirtualKeyCode, F>,
}

pub struct Controller<'a, T: 'a> {
	pub controlled: &'a mut T,
	pub controller: 
}
