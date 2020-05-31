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

	// TODO: Don't just return the rect!!!
	pub fn bounds(&self) -> Rectangle {
		self.bounds
	}
}

impl Clone for Collider {
	fn clone(&self) -> Self {
		Collider { bounds: self.bounds }
	}
}
