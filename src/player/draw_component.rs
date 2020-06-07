use crate::physics::Bounds;

use quicksilver::{
	geom::{Circle, Rectangle, Vector},
	graphics::{Color, Image},
	Graphics,
};

pub struct DrawComponent {
	image: Option<Image>,
}

impl DrawComponent {
	pub fn new(image: Option<Image>) -> Self {
		DrawComponent { image }
	}

	pub fn draw(&self, bounds: &dyn Bounds, gfx: &mut Graphics) {
		// TODO: No if statement, just use a different component or None
		if let Some(image) = &self.image {
			gfx.draw_image(
				&image,
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
