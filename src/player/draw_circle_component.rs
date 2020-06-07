use crate::{components::draw_component::DrawComponent, physics::Bounds};
use quicksilver::{geom::Circle, graphics::Color, Graphics};

pub struct DrawCircleComponent;

impl DrawCircleComponent {
	pub fn new() -> Self {
		DrawCircleComponent {}
	}
}

impl DrawComponent for DrawCircleComponent {
	fn draw(&self, gfx: &mut Graphics, bounds: Option<&dyn Bounds>) {
		let bounds = bounds.expect("Where are your bounds bruh??");

		gfx.fill_circle(
			&Circle::new(bounds.pos(), bounds.radius()),
			Color::from_hex("4f30d9"),
		);
	}
}
