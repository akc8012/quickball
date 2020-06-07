pub mod raycast;

use quicksilver::{
	geom::{Rectangle, Vector},
	graphics::Color,
	Graphics,
};

pub trait Bounds {
	fn x(&self) -> f32;

	fn y(&self) -> f32;

	fn top_left(&self) -> Vector;

	fn top_right(&self) -> Vector;

	fn width(&self) -> f32;

	// TODO: THIS SHOULD BE ON A DIFFERENT TRAIT
	fn draw(&self, gfx: &mut Graphics);
}

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

impl Clone for RectangleBounds {
	fn clone(&self) -> Self {
		RectangleBounds { bounds: self.bounds }
	}
}

pub struct PointBounds {
	point: Vector,
}

impl PointBounds {
	pub fn new(point: Vector) -> Self {
		Self {
			point: (point.x.round(), point.y.round()).into(),
		}
	}
}

impl Bounds for PointBounds {
	fn x(&self) -> f32 {
		self.point.x
	}

	fn y(&self) -> f32 {
		self.point.y
	}

	fn top_left(&self) -> Vector {
		self.point
	}

	fn top_right(&self) -> Vector {
		self.point
	}

	fn width(&self) -> f32 {
		1.
	}

	fn draw(&self, gfx: &mut Graphics) {
		gfx.draw_point(self.point, Color::GREEN);
	}
}
