use crate::physics::Bounds;

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

	pub fn draw(&self, bounds: &dyn Bounds, image: &Option<Image>, gfx: &mut Graphics) {
		if let Some(image) = image {
			gfx.draw_image(
				image,
				Rectangle::new(bounds.pos() - (Vector::ONE * bounds.radius()), image.size()),
			);
		} else {
			gfx.fill_circle(
				&Circle::new(bounds.pos(), bounds.radius()),
				Color::from_hex("4f30d9"),
			);
		}
	}
}
