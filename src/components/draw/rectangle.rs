use super::DrawComponent;
use crate::physics::Bounds;
use quicksilver::{geom::Rectangle, graphics::Color, Graphics};

pub struct DrawRectangleComponent {
	color: Color,
}

impl DrawRectangleComponent {
	pub fn new(color: Color) -> Self {
		DrawRectangleComponent { color }
	}
}

impl DrawComponent for DrawRectangleComponent {
	fn draw(&self, gfx: &mut Graphics, bounds: Option<&dyn Bounds>) {
		let bounds = bounds.expect("Where are your bounds bruh??");

		gfx.fill_rect(&Rectangle::new(bounds.pos(), bounds.size()), self.color);
	}
}
