#[macro_use] extern crate conrod;
use conrod::backend::glium::glium::{self, Surface};

fn main() {
	const WIDTH: u32 = 500;
	const HEIGHT: u32 = 500;

	let mut events_loop = glium::glutin::EventsLoop::new();
	let window = glium::glutin::WindowBuilder::new()
					.with_title("GUI")
					.with_dimensions(WIDTH, HEIGHT);
	let context = glium::glutin::ContextBuilder::new()
					.with_vsync(true)
					.with_multisampling(4);
	let display = glium::Display::new(window, context, &events_loop).unwrap();
	let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();
	let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();
	let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();
	 widget_ids!(struct Ids { canvas, button, rust_logo });
			let ids = Ids::new(ui.widget_id_generator());

			
	// Button widget example button.
	if widget::Button::image(image_ids.normal)
		.hover_image(image_ids.hover)
		.press_image(image_ids.press)
		.w_h(w as conrod::Scalar, h as conrod::Scalar)
		.middle_of(ids.canvas)
		.border(0.0)
		.set(ids.button, ui)
		.was_clicked()
		
	'main: loop {
		// Render the `Ui` and then display it on the screen.
		if let Some(primitives) = ui.draw_if_changed() {
			renderer.fill(&display, primitives, &image_map);
			let mut target = display.draw();
			target.clear_color(0.0, 1.0, 0.0, 1.0);
			renderer.draw(&display, &mut target, &image_map).unwrap();
			target.finish().unwrap();
		}
	}
}


