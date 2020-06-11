pub mod circle;
pub mod image;
pub mod rectangle;

use crate::physics::Bounds;
use quicksilver::Graphics;

pub trait DrawComponent {
	// TODO: Should Bounds be non-optional? (probably)
	fn draw(&self, gfx: &mut Graphics, bounds: Option<&dyn Bounds>);
}
