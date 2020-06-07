use quicksilver::{
	geom::{Circle, Rectangle, Vector},
	graphics::{Color, Image},
	Graphics,
};

pub struct DrawComponent;

impl DrawComponent {
	pub fn new() -> Self {
		DrawComponent {}
	}

	pub fn draw(&self, pos: &Vector, radius: f32, image: &Option<Image>, gfx: &mut Graphics) {
		if let Some(image) = image {
			gfx.draw_image(image, Rectangle::new(*pos - (Vector::ONE * radius), image.size()));
		} else {
			gfx.fill_circle(&Circle::new(*pos, radius), Color::from_hex("4f30d9"));
		}
	}
}
