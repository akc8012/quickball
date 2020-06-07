pub mod colliders;
pub mod raycast;

use quicksilver::{geom::Vector, Graphics};

pub trait Bounds {
	fn x(&self) -> f32;
	fn y(&self) -> f32;
	fn pos(&self) -> Vector;

	fn top_left(&self) -> Vector;
	fn top_right(&self) -> Vector;

	fn width(&self) -> f32;
	fn radius(&self) -> f32;

	fn set_pos(&mut self, pos: Vector) -> Vector;
	fn set_x(&mut self, x: f32) -> f32;
	fn set_y(&mut self, y: f32) -> f32;

	// TODO: Should not be on Bounds trait, instead: draw within Colliders?
	fn draw(&self, gfx: &mut Graphics);
}
