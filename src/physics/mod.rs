pub mod raycast;

use quicksilver::{
	geom::{Rectangle, Vector},
	graphics::Color,
	Graphics,
};

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

	fn pos(&self) -> Vector {
		self.point
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

	fn radius(&self) -> f32 {
		panic!("Attempt to get the radius of a PointBounds")
	}

	fn set_pos(&mut self, pos: Vector) -> Vector {
		self.point = pos;
		self.point
	}

	fn set_x(&mut self, x: f32) -> f32 {
		self.point.x = x;
		self.point.x
	}

	fn set_y(&mut self, y: f32) -> f32 {
		self.point.y = y;
		self.point.y
	}

	fn draw(&self, gfx: &mut Graphics) {
		gfx.draw_point(self.point, Color::GREEN);
	}
}

pub struct CircleBounds {
	pub pos: Vector,
	pub radius: f32,
}

impl CircleBounds {
	pub fn new(pos: Vector, radius: f32) -> Self {
		Self { pos, radius }
	}
}

impl Bounds for CircleBounds {
	fn x(&self) -> f32 {
		self.pos.x
	}

	fn y(&self) -> f32 {
		self.pos.y
	}

	fn pos(&self) -> Vector {
		self.pos
	}

	fn top_left(&self) -> Vector {
		todo!()
	}

	fn top_right(&self) -> Vector {
		todo!()
	}

	fn width(&self) -> f32 {
		self.radius * 2.
	}

	fn radius(&self) -> f32 {
		self.radius
	}

	fn set_pos(&mut self, pos: Vector) -> Vector {
		self.pos = pos;
		self.pos
	}

	fn set_x(&mut self, x: f32) -> f32 {
		self.pos.x = x;
		self.pos.x
	}

	fn set_y(&mut self, y: f32) -> f32 {
		self.pos.y = y;
		self.pos.y
	}

	fn draw(&self, _gfx: &mut Graphics) {
		todo!()
	}
}
