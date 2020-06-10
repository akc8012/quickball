use super::DrawComponent;
use crate::physics::Bounds;
use quicksilver::{geom::Circle, graphics::Color, Graphics};

pub struct DrawCircleComponent {
	color: Color,
}

impl DrawCircleComponent {
	pub fn new(color: Color) -> Self {
		DrawCircleComponent { color }
	}
}

impl DrawComponent for DrawCircleComponent {
	fn draw(&self, gfx: &mut Graphics, bounds: Option<&dyn Bounds>) {
		let bounds = bounds.expect("Where are your bounds bruh??");

		gfx.fill_circle(&Circle::new(bounds.pos(), bounds.radius()), self.color);
	}
}
