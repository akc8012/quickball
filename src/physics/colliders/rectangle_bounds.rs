use crate::physics::Bounds;

use quicksilver::{
	geom::{Rectangle, Vector},
	graphics::Color,
	Graphics,
};

pub struct RectangleBounds {
	bounds: Rectangle,
}

impl RectangleBounds {
	pub fn new(pos: impl Into<Vector>, size: impl Into<Vector>) -> Self {
		let mut bounds = Rectangle::new(pos, size);
		bounds.pos = (bounds.pos.x.round(), bounds.pos.y.round()).into();
		bounds.size = (bounds.size.x.round(), bounds.size.y.round()).into();

		Self { bounds }
	}
}

impl Bounds for RectangleBounds {
	fn x(&self) -> f32 {
		self.bounds.x()
	}

	fn y(&self) -> f32 {
		self.bounds.y()
	}

	fn pos(&self) -> Vector {
		self.bounds.pos
	}

	fn top_left(&self) -> Vector {
		self.bounds.top_left()
	}

	fn top_right(&self) -> Vector {
		(self.x() + self.width(), self.y()).into()
	}

	fn width(&self) -> f32 {
		self.bounds.width()
	}

	fn radius(&self) -> f32 {
		self.bounds.width() / 2.
	}

	fn set_pos(&mut self, pos: Vector) -> Vector {
		self.bounds.pos = pos;
		self.bounds.pos
	}

	fn set_x(&mut self, x: f32) -> f32 {
		self.bounds.pos.x = x;
		self.bounds.pos.x
	}

	fn set_y(&mut self, y: f32) -> f32 {
		self.bounds.pos.y = y;
		self.bounds.pos.y
	}

	fn draw(&self, gfx: &mut Graphics) {
		gfx.fill_rect(&self.bounds, Color::GREEN);
	}
}
