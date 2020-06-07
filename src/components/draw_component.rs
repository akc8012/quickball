use crate::physics::Bounds;
use quicksilver::Graphics;

pub trait DrawComponent {
	// TODO: lol should physics::Bounds be in DrawComponent?
	fn draw(&self, gfx: &mut Graphics, bounds: Option<&dyn Bounds>);
}
