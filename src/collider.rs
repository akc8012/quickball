use quicksilver::geom::{Rectangle, Vector};

pub struct Collider {
	bounds: Rectangle,
}

impl Collider {
	pub fn new(pos: impl Into<Vector>, size: impl Into<Vector>) -> Self {
		Self {
			bounds: Rectangle::new(pos, size),
		}
	}

	pub fn x(&self) -> f32 {
		self.bounds.x()
	}

	pub fn y(&self) -> f32 {
		self.bounds.y()
	}

	pub fn top_right(&self) -> f32 {
		self.x() + self.width()
	}

	pub fn width(&self) -> f32 {
		self.bounds.width()
	}
}

impl Clone for Collider {
	fn clone(&self) -> Self {
		Collider { bounds: self.bounds }
	}
}
