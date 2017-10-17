use std::collections::HashMap;

use glutin;
use glutin::WindowEvent;

use transform::Transform;

pub enum MouseInput {
	Moved(glutin::DeviceID, glutin::ElementState, ),
	Wheel(),
	Left(),
	Right(),
	Input(),
}

pub enum UserInput {
	Keyboard(),
	Mouse(MouseInput),
}

pub trait ControlHandler {
	
}

pub struct Controller<'a, T: 'a, H: ControlHandler> {
	pub controlled: &'a mut T,
	pub controller: H,
}
