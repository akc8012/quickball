use crate::physics::Bounds;
use quicksilver::Graphics;

pub trait DrawComponent {
	fn draw(&self, gfx: &mut Graphics, bounds: Option<&dyn Bounds>);
}
