use crate::components::draw_component;
use crate::physics::Bounds;

use quicksilver::{
	geom::{Rectangle, Vector},
	graphics::Image,
	Graphics,
};

pub struct DrawImageComponent {
	image: Image,
}

impl DrawImageComponent {
	pub fn new(image: Image) -> Self {
		DrawImageComponent { image }
	}
}

impl draw_component::DrawComponent for DrawImageComponent {
	fn draw(&self, gfx: &mut Graphics, bounds: Option<&dyn Bounds>) {
		let bounds = bounds.expect("Where are your bounds bruh??");

		gfx.draw_image(
			&self.image,
			Rectangle::new(bounds.pos() - (Vector::ONE * bounds.radius()), self.image.size()),
		);
	}
}
