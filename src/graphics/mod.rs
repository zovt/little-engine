use glutin::{ContextBuilder, CreationError, EventsLoop, GlWindow, WindowBuilder};

pub fn create_window(name: &str) -> Result<(GlWindow, EventsLoop), CreationError> {
	let ev_loop = EventsLoop::new();
	let (w, h) = ev_loop.get_primary_monitor().get_dimensions();
	let window = WindowBuilder::new().with_title(name).with_dimensions(w, h);
	let context = ContextBuilder::new();
	GlWindow::new(window, context, &ev_loop)
		.map(|w| (w, ev_loop))
}
