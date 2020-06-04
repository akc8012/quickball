use quicksilver::{
	geom::{Rectangle, Vector},
	graphics::Color,
	Graphics,
};

pub trait Collide {
	fn x(&self) -> f32;

	fn y(&self) -> f32;

	fn top_left(&self) -> Vector;

	fn top_right(&self) -> Vector;

	fn width(&self) -> f32;

	// TODO: THIS SHOULD BE ON A DIFFERENT TRAIT
	fn draw(&self, gfx: &mut Graphics);
}

pub struct RectangleCollider {
	bounds: Rectangle,
}

impl RectangleCollider {
	pub fn new(pos: impl Into<Vector>, size: impl Into<Vector>) -> Self {
		Self {
			bounds: Rectangle::new(pos, size),
		}
	}
}

impl Collide for RectangleCollider {
	fn x(&self) -> f32 {
		self.bounds.x()
	}

	fn y(&self) -> f32 {
		self.bounds.y()
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

	fn draw(&self, gfx: &mut Graphics) {
		gfx.fill_rect(&self.bounds, Color::GREEN);
	}
}

impl Clone for RectangleCollider {
	fn clone(&self) -> Self {
		RectangleCollider { bounds: self.bounds }
	}
}
