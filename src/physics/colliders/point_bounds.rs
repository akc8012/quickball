use crate::physics::Bounds;
use quicksilver::{geom::Vector, graphics::Color, Graphics};

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
