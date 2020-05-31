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

	pub fn top_left(&self) -> Vector {
		self.bounds.top_left()
	}

	pub fn top_right(&self) -> Vector {
		(self.x() + self.width(), self.y()).into()
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
